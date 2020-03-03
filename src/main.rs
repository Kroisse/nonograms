// Nonograms
// Copyright (C) 2020  Eunchong Yu <kroisse at gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
#![recursion_limit = "256"]

mod components;
mod ext;
mod game;
mod model;

use crate::model::Model;

fn main() {
    pretty_env_logger::init();
    std::process::exit(vgtk::run::<Model>());
}
