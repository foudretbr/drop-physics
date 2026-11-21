# Notices

Drop Physics is licensed under the GNU Lesser General Public License, version 3 or later (see `LICENSE`, which
adds permissions to the GNU General Public License version 3, in `COPYING`).

## Horizon

`src/mixins/items.rs` reproduces and adapts the body of Horizon's dropped-item drawing, `dropped_item_geometry`
and `item_entity_geometry` in `crates/horizon-client/src/game/interface/item_draw.rs`, at
https://github.com/horizon-rust/horizon/blob/8f83e5c85ae3b9a117f742e8d91170c5c9080a06/crates/horizon-client/src/game/interface/item_draw.rs
(Horizon, https://github.com/horizon-rust/horizon, GNU LGPL version 3 or later, by Ludovic Marie, the Horizon Team
and its contributors). Drop Physics is a modified version of that code and is therefore distributed under the same
licence. Its changes: the item pose (tumbling in flight, settling on the flattest side), in `items.rs` and
`runtime.rs`.

## Horizon Physics (MIT)

The approach follows Horizon Physics by tariksabanuz (https://github.com/tariksabanuz/PhysicsModHorizon), under the
MIT License with this notice. The full MIT text is at the end of this file.

Copyright (c) 2026 tariksabanuz

## Author

Copyright (c) 2026 foudretbr

## MIT License text (Horizon Physics)
Horizon was released under the MIT licence until it moved to the GNU LGPL (see LICENSE).
Code merged before that change stays available under these terms:

MIT License

Copyright (c) 2026 Ludovic Marie and the Horizon contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

