// Garta - GPX editor and analyser
// Copyright (C) 2016  Timo Saarinen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate gtk;
extern crate gio;
extern crate glib;

use std::cell::{RefCell};
use std::rc::{Rc};
use std::result::*;
use std::collections::linked_list::LinkedList;
use self::gtk::prelude::*;
use self::glib::variant::{FromVariant};
use core::geo::{Location};
use core::root::{Atlas, MapView};
use core::id::{UniqueId};
//use core::settings::{settings_read};

/// Main window..
pub struct MapWindow {
    atlas: Rc<RefCell<Atlas>>,
    map_view: MapView,
    
    win: Option<gtk::ApplicationWindow>,
    coordinates_label: Option<gtk::Label>,
    zoom_level_label: Option<gtk::Label>,
    maps_button: Option<gtk::MenuButton>,
    layers_button: Option<gtk::MenuButton>,
    layers_button_label: Option<Rc<RefCell<gtk::Label>>>,
    coordinates_button: Option<gtk::MenuButton>,    
}

impl Clone for MapWindow {
    fn clone(&self) -> MapWindow {
        MapWindow {
            atlas: self.atlas.clone(),
            map_view: self.map_view.clone(),
            win: self.win.clone(),
            coordinates_label: self.coordinates_label.clone(),
            zoom_level_label: self.zoom_level_label.clone(),
            maps_button: self.maps_button.clone(),
            layers_button: self.layers_button.clone(),
            layers_button_label: self.layers_button_label.clone(),
            coordinates_button: self.coordinates_button.clone(),
        }
    }
}

impl MapWindow {
    pub fn new(atlas: Rc<RefCell<Atlas>>, map_view: MapView) -> MapWindow {
        MapWindow {
            atlas: atlas,
            map_view: map_view,
            win: None,
            coordinates_label: None,
            zoom_level_label: None,
            maps_button: None,
            layers_button: None,
            layers_button_label: None,
            coordinates_button: None,
        }
    }

    /// Build main window and run GTK main.
    pub fn run(&mut self) -> Result<(), &'static str> {
        // Initialize GTK
        if gtk::init().is_err() {
            return Err("Failed to initialize GTK.");
        }

        // Load resources from a glade file
        let builder = gtk::Builder::new_from_file("ui/main-window.ui");
        self.win = builder.get_object("main_window");
        if let Some(ref mut win) = self.win {
            // Action for add_attraction
            let add_attraction_action = gio::SimpleAction::new("add_attraction", None);
            add_attraction_action.connect_activate(|_, _| {
                println!("add_attraction");
            });
            win.add_action(&add_attraction_action);
            
            // Action for add_waypoint
            let add_waypoint_action = gio::SimpleAction::new("add_waypoint", None);
            add_waypoint_action.connect_activate(|_, _| {
                println!("add_waypoint");
            });
            win.add_action(&add_waypoint_action);
            
            // Action for add_track
            let add_track_action = gio::SimpleAction::new("add_track", None);
            add_track_action.connect_activate(|_, _| {
                println!("add_track");
            });
            win.add_action(&add_track_action);
            
            // Action for add_route
            let add_route_action = gio::SimpleAction::new("add_route", None);
            add_route_action.connect_activate(|_, _| {
                println!("add_route");
            });
            win.add_action(&add_route_action);
            
            // Action for manage_layers
            let add_layers_action = gio::SimpleAction::new("manage_layers", None);
            add_layers_action.connect_activate(|_, _| {
                println!("manage_layers");
            });
            win.add_action(&add_layers_action);
            
            // Assign components to the struct
            self.coordinates_label = Some(builder.get_object("coordinates_button_label").unwrap());
            self.zoom_level_label = Some(builder.get_object("zoom_level_label").unwrap());
            self.maps_button = Some(builder.get_object("maps_button").unwrap());
            self.layers_button = Some(builder.get_object("layers_button").unwrap());
            self.layers_button_label = Some(Rc::new(RefCell::new(builder.get_object("layers_button_label").unwrap())));
            self.coordinates_button = Some(builder.get_object("coordinates_button").unwrap());
            
            // Event for window close button
            win.connect_delete_event(|_, _| {
                gtk::main_quit();
                gtk::Inhibit(false)
            });
            
            // Show win and enter GTK main loop
            win.show_all();
        } else {
            return Err("No main_window object found in the resource file.");
        }
        
        // Populate popovers and override default values
        let zoom_level = self.map_view.zoom_level;
        self.populate_maps_button();
        self.populate_layers_button();
        self.populate_coordinates_button();
        self.update_layers_button();
        self.update_coordinates_button(None);
        self.update_zoom_level_label(zoom_level);
        self.update_map();
        
        // Start main
        gtk::main();
        Ok(())
    }

    /// Populate (or re-populate) maps button popover.
    pub fn populate_maps_button(&mut self) {
        if let Some(ref button) = self.maps_button {
            if let Some(ref win) = self.win {
                let menu_model = gio::Menu::new();
                
                // Choose map action
                let action = gio::SimpleAction::new_stateful(
                                "choose_map", 
                                Some(&glib::VariantType::new("s").unwrap()),
                                &"default".to_string().to_variant()
                                );
                let map_win = RefCell::new(self.clone());
                action.connect_activate( move |action, map_id_variant| {
                    if let Some(ref var) = *map_id_variant {
                        if let Some(map_id_str) = var.get_str() {
                            let map_id = map_id_str.parse::<UniqueId>().unwrap();
                            println!("choose_map action invoked {}!", map_id);
                            action.set_state(var);
                            
                            // Change map on the view
                            let mut mwin = map_win.borrow_mut();
                            {
                                let mut atlas = mwin.atlas.borrow_mut();
                                if let Some(backdrop_layer_id) = atlas.backdrop_layer_id() {
                                    atlas.layers.get_mut(&backdrop_layer_id).unwrap().map_id = map_id;
                                }
                            }
                            
                            // Refresh the map element
                            mwin.update_map();
                        }
                    }
                });
                win.add_action(&action);

                // Fill in and add the maps section
                let atlas = self.atlas.borrow();
                for (_, map) in &atlas.maps {
                    if !map.transparent {
                        let item = gio::MenuItem::new(
                            Some(map.name.as_str()), 
                            Some(format!("win.choose_map('{}')", map.id()).as_str()));
                        menu_model.append_item(&item);
                    }
                }
                action.set_state(&"map1".to_string().to_variant()); // TODO

                // Set menu model                
                button.set_menu_model(Some(&menu_model));
            }
        }        
    }

    /// Populate (or re-populate) layers button popover.
    pub fn populate_layers_button(&mut self) {
        if let Some(ref button) = self.layers_button { if let Some(ref win) = self.win {
            let menu_model = gio::Menu::new();
            
            // Layers section
            let atlas = self.atlas.borrow();
            let layers_section = gio::Menu::new();
            for (layer_id, layer) in &atlas.layers {
                let initial_state = self.map_view.visible_layer_ids
                    .iter()
                    .filter(|&la_id| la_id == layer_id)
                    .count() > 0;
                
                // Only transparent layers are toggleable
                if layer.backdrop { continue; }
                
                // Choose map action
                let action = gio::SimpleAction::new_stateful(
                                format!("toggle_layer_{}", layer_id).as_ref(), 
                                None,
                                &initial_state.to_variant()
                                );

                // Action closure
                let map_win = RefCell::new(self.clone());
                action.connect_change_state( move |action, value| {
                    let layer_id_str = &action.get_name().unwrap()["toggle_layer_".len()..];
                    let layer_id = layer_id_str.parse::<UniqueId>().unwrap();
                    if let Some(ref var) = *value {
                        println!("toggle_layer({}) action invoked {}!", layer_id, var);
                        if let Some(var_bool) = bool::from_variant(var) {
                            action.set_state(var);

                            // Change layers list of map view
                            let mut win = map_win.borrow_mut();
                            {
                                let atlas_rr = win.atlas.clone();
                                let atlas = atlas_rr.borrow_mut();
                                
                                for (layer_id, layer) in &atlas.layers {
                                    if layer_id == layer_id {
                                        if var_bool {
                                            // Add layer
                                            win.map_view.visible_layer_ids.push_back(*layer_id);
                                        } else {
                                            // Drop layer
                                            win.map_view.visible_layer_ids = win.map_view.visible_layer_ids
                                                .iter()
                                                .filter(|&la_id| la_id != layer_id)
                                                .cloned()
                                                .collect::<LinkedList<UniqueId>>();
                                        }
                                    }
                                }
                            }
                            
                            // Change layer label
                            win.update_layers_button();
                            
                            // Refresh the map element
                            win.update_map();
                        }
                    }
                });
                win.add_action(&action);

                // Menu item
                let item = gio::MenuItem::new(
                    Some(layer.name.as_str()), 
                    Some(format!("win.toggle_layer_{}", layer.id()).as_str()));
                layers_section.append_item(&item);
            }
            menu_model.append_section(None, &layers_section);

            // Add manage item to ops section
            let ops_section = gio::Menu::new(); 
            ops_section.append_item(&gio::MenuItem::new(
                Some("Manage..."), 
                Some("win.manage_layers")));
            menu_model.append_section(None, &ops_section);

            // Set menu model                
            button.set_menu_model(Some(&menu_model));
        } }   
    }

    /// Populate coordinates button popover.
    pub fn populate_coordinates_button(&mut self) {
        if let Some(ref button) = self.coordinates_button {
            if let Some(ref win) = self.win {
                let menu_model = gio::Menu::new();
                
                // Choose coordinates action
                let action = gio::SimpleAction::new_stateful(
                                "choose_coordinates", 
                                Some(&glib::VariantType::new("s").unwrap()),
                                &"default".to_string().to_variant(),
                                );
                action.connect_activate( |action, coordinates_slug_variant| {
                    if let Some(ref var) = *coordinates_slug_variant {
                        if let Some(coordinates_slug) = var.get_str() {
                            println!("choose_cordinates action invoked {}!", coordinates_slug);
                            action.set_state(var);
                            
                            // TODO: change coordinates on the view
                        }
                    }
                });
                win.add_action(&action);

                // Fill in and add the coordinates section
                menu_model.append_item(&gio::MenuItem::new(
                    Some("DDD°mm'SS.ss\""), Some("win.choose_coordinates('dms')")));
                menu_model.append_item(&gio::MenuItem::new(
                    Some("DDD°mm.mmm°"), Some("win.choose_coordinates('dm')")));
                menu_model.append_item(&gio::MenuItem::new(
                    Some("DDD.ddddd°"), Some("win.choose_coordinates('d')")));
                menu_model.append_item(&gio::MenuItem::new(
                    Some("-DDD.ddddd"), Some("win.choose_coordinates('-d')")));
                action.set_state(&"dm".to_string().to_variant()); // TODO

                // Set menu model                
                button.set_menu_model(Some(&menu_model));
            }
        }
    }

    /// Update layers button label.
    pub fn update_layers_button(&mut self) {
        if let Some(ref label) = self.layers_button_label {
            let atlas = self.atlas.borrow();
            let m = self.map_view.visible_layer_ids.len();
            let n = atlas.layers.len();
            label.borrow().set_text(format!("Layers {}/{}", m, n).as_str());
        }
    }

    /// Update coordinates in the coordinates button.
    pub fn update_coordinates_button(&mut self, location: Option<Location>) {
        if let Some(ref label) = self.coordinates_label {
            if let Some(ref loc) = location {
                label.set_text(loc.to_string().as_ref());
            } else {
                label.set_text("--°N --°E");
            }
        }
    }

    /// Update zoom level label.
    pub fn update_zoom_level_label(&mut self, zoom_level: u8) {
        if let Some(ref label) = self.zoom_level_label {
            if zoom_level > 0 {
                label.set_text(format!("L{}", zoom_level).as_ref());
            } else {
                label.set_text("--");
            }
        }
    }
    
    /// Refresh the map element.
    pub fn update_map(&mut self) {
        // TODO
    }
}

