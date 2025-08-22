///
/// @package halloween
///
/// @file Main functions
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}