fn drop_physics_items(game: &crate::game::Game<'_>, camera: glam::DVec3, alpha: f64, out: &mut Vec<Vertex>) {
    mod_drop_physics::runtime::begin_frame();
    let items = game.dropped.iter().map(|item| {
        let (p, q) = (item.previous, item.position);
        let at = [
            p.x + (q.x - p.x) * alpha,
            p.y + (q.y - p.y) * alpha,
            p.z + (q.z - p.z) * alpha,
        ];
        ItemEntityDraw {
            item,
            at,
            partial: alpha as f32,
            entity: true,
        }
    });
        let range =
            super::entity_culling::default_range(glam::DVec3::splat(0.25), game.view_scale());
        let skinning = game.skinning();
        let lights = diffuse_lights(game.dim == Dim::Nether);
        for ItemEntityDraw {
            item,
            at,
            partial,
            entity,
        } in items
        {
            let entity_start = out.len();
            game.mark_motion(entity_start, (2 << 32) | u64::from(item.id));
            let q = item.position;
            let pos = glam::DVec3::from_array(at) - camera;
            let far = if entity {
                (glam::DVec3::new(q.x, q.y, q.z) - camera).length_squared() >= range * range
            } else {
                pos.length_squared() > 64.0 * 64.0
            };
            if far {
                continue;
            }
            if entity && let Some(view) = &game.view_frustum {
                let (lo, hi) = dropped_item_cull_box(glam::DVec3::from_array(at), camera);
                if !view.intersects(lo, hi) {
                    continue;
                }
            }
            let owner = owner_at([q.x, q.y, q.z], 0.0, "minecraft:item");
            let state = game.resolve_item(
                &item.stack,
                DisplayContext::Ground,
                Some(&owner),
                item.id as i32,
            );
            if state.is_empty() {
                continue;
            }
            let age = item.age as f32 + partial;
            let (lo, hi) = state.bounding_box();
            let min_offset_y = -lo.y + 0.0625;
            let bob = (age / 10.0 + item.bob_offset).sin() * 0.1 + 0.1;
            let spin = item_render::item_spin(age, item.bob_offset);
            let size = hi - lo;
            let flat = size.z < 0.25 * size.x.max(size.y);
            let (prev, now) = (item.previous, item.position);
            let velocity = [
                ((now.x - prev.x) * 20.0) as f32,
                ((now.y - prev.y) * 20.0) as f32,
                ((now.z - prev.z) * 20.0) as f32,
            ];
            let resting = item.on_ground
                || ((now.y - prev.y).abs() < 1.0e-4 && item.velocity.y.abs() < 1.0e-3);
            let r = mod_drop_physics::runtime::rotation(item.id, resting, velocity, flat);
            let rot = glam::Quat::from_xyzw(r[0], r[1], r[2], r[3]);
            let center = (lo + hi) * 0.5;
            let half = (hi - lo) * 0.5;
            let below = (rot * glam::Vec3::new(half.x, 0.0, 0.0)).y.abs()
                + (rot * glam::Vec3::new(0.0, half.y, 0.0)).y.abs()
                + (rot * glam::Vec3::new(0.0, 0.0, half.z)).y.abs();
            let pose = glam::Mat4::from_translation(pos.as_vec3())
                * glam::Mat4::from_translation(glam::Vec3::new(0.0, below + 0.01, 0.0))
                * glam::Mat4::from_quat(rot)
                * glam::Mat4::from_translation(-center);
            let packed = if entity {
                let probe = [
                    at[0],
                    at[1] + horizon_client::entities::ItemEntity::EYE_HEIGHT,
                    at[2],
                ];
                item.render_light(game.world.light(
                    probe[0].floor() as i32,
                    probe[1].floor() as i32,
                    probe[2].floor() as i32,
                ))
            } else {
                vertex_light(game.world.light(
                    q.x.floor() as i32,
                    (q.y + 0.1).floor() as i32,
                    q.z.floor() as i32,
                ))
            };
            let seed = i64::from(game.pack.item_renderer.item_id(&item.stack.item))
                + i64::from(item.stack.damage);
            let copies = item_render::cluster_poses_with_depth(
                pose,
                item_render::rendered_amount(item.stack.count),
                seed,
                hi.z - lo.z,
            );
            let culling = entity.then(|| {
                if game.settings.horizon.entity_culling == EntityCullingMode::Off {
                    return EntityCullDecision::Bypass;
                }
                state
                    .posed_bounds(&copies)
                    .map(|(lo, hi)| {
                        game.entity_cull_decision_in_bounds(
                            item,
                            Aabb::new(
                                Vec3::new(
                                    camera.x + f64::from(lo.x),
                                    camera.y + f64::from(lo.y),
                                    camera.z + f64::from(lo.z),
                                ),
                                Vec3::new(
                                    camera.x + f64::from(hi.x),
                                    camera.y + f64::from(hi.y),
                                    camera.z + f64::from(hi.z),
                                ),
                            ),
                        )
                    })
                    .unwrap_or_else(|| game.entity_cull_decision(item, f64::from(partial)))
            });
            if culling.is_some_and(|decision| decision.hides_entity()) {
                continue;
            }
            let outline = game.thing_outline(item.id, "minecraft:item");
            let skinned = skinning
                .then(|| game.submit_item_skinned(&state, &copies, packed, lights))
                .flatten();
            if skinned.is_none() {
                for &copy in &copies {
                    game.submit_item_world(&state, copy, packed, out);
                }
            }
            if let Some(rgb) = outline {
                game.mark_outline(&out[entity_start..], rgb);
                if let Some(((first, _, _), _)) = skinned {
                    let color = horizon_client::mobs::render::abgr(rgb);
                    game.skin_frame.borrow_mut().outline_since(first, color);
                }
            } else if let Some(culling) = culling {
                game.finish_entity_occlusion(culling, item, out, entity_start);
            } else {
                game.cull_skinned(out, entity_start, skinned);
            }
        }
    mod_drop_physics::runtime::end_frame();
}


