// Garta - GPX viewer and editor
// Copyright (C) 2016-2017, Timo Saarinen
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

use std::cell::{RefCell};
use std::rc::{Rc};

use core::atlas::*;
use core::id::*;

use geocoord;
use geocoord::geo::{GeoBox, Location};

// ---- MapElement ---------------------------------------------------------------------------------

// Based on my novice question on StackOverflow; http://stackoverflow.com/questions/40963710/extended-traits-in-collections

pub trait MapElement {
    /// Returns the unique id of the element
    fn id(&self) -> UniqueId;
    
    /// Returns bounding box of the element.
    fn bounding_box(&self) -> GeoBox;

    /// True if this an element from a remote layer.
    fn is_remote(&self) -> bool { false }
}

/*
impl Ord for MapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bounding_box().cmp(&other.bounding_box())
    }
}

impl PartialOrd for MapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapElement {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for MapElement {}
*/

// ---- Attraction ---------------------------------------------------------------------------------

/// A simple point-like destination on the map, also known as a landmark.
pub struct Attraction { 
    id: UniqueId,
    pub location: Location,
}

impl Attraction {
    /// Constructor.
    pub fn new(loc: Location) -> Attraction {
        Attraction {
            id: super::id::next_id(),
            location: loc,
        }
    }
}

impl MapElement for Attraction {
    /// Id getter.    
    fn id(&self) -> UniqueId { self.id }
    
    fn bounding_box(&self) -> GeoBox {
        GeoBox::new(self.location, self.location)
    }
}

// ---- Path ---------------------------------------------------------------------------------------

//
// When a GPX file with several items is loaded, the user will be given options on
// whether to load them to a new layer or to merge to an existing one
//
// Later, layers can be exported as GPX files, and attractions become waypoints.
//

// -------------------------------------------------------------------------------------------------

/// Load GPX data from file to a given layer
pub fn load_from_file(gpx_filename: String, layer: &Rc<RefCell<Layer>>) {
    // TODO
}

// Save the given layer to a GPX file
pub fn save_layer(gpx_filename: String, layer: &Rc<RefCell<Layer>>) {
    // TODO
}

// ---- Waypoint -----------------------------------------------------------------------------------

/// GPX waypoint
pub struct Waypoint { 
    id: UniqueId,
    location: Location,
}

impl Waypoint {
    /// Constructor.
    pub fn new(loc: Location) -> Waypoint {
        Waypoint {        
            id: super::id::next_id(),
            location: loc,
        }
    }
}

impl MapElement for Waypoint {
    fn bounding_box(&self) -> GeoBox {
        GeoBox::new(self.location, self.location)
    }
    
    /// Id getter.    
    fn id(&self) -> UniqueId { self.id }
    
}

// ---- Path ---------------------------------------------------------------------------------------

/// GPX routes and tracks.
pub enum PathMode {
    Neither,
    PathTrack { track: geocoord::gpx::model::Track },
    PathRoute { route: geocoord::gpx::model::Route },
}

pub struct Path {
    id: UniqueId,
    mode: PathMode,
}

impl Path {
    /// Create a new empty layer.
    pub fn new(slug: String) -> Path {
        Path{
            id: super::id::next_id(),
            mode: PathMode::Neither,
        }    
    }
}

impl Path {
    /// Remove idle points from the beginning and end of the path.
    pub fn trim(&mut self, radius: f64) {
    }
    
    /// Remove points that have too high acceleration (or decceleration).
    pub fn limit_acceleration(&mut self, max_acceleration: f64) {
    }
    
    /// Find idle spots on the track and split it to legs when found.
    pub fn divide_on_idle(&mut self, radius: f64, delay: f64) {
    }

    /// Join legs if their end and start time is lesser than the given.
    pub fn join_legs(&mut self, max_time: f64) {
    }

    /// drop points that make the track too sharp.    
    pub fn smooth(&mut self, max_angle: f64) {
    }
    
    /// Drop points to make the tracke sparser.
    pub fn make_sparser(&mut self, min_distance: f64) {
//        for leg in self.legs.iter_mut() {
//            for point in leg.borrow().points.iter() {
//                // TODO
//            }
//        }
    }
}

impl MapElement for Path {
    fn bounding_box(&self) -> GeoBox {
        GeoBox::new(Location::new(0.0, 0.0), Location::new(0.0, 0.0)) // TODO
    }
    
    /// Id getter.    
    fn id(&self) -> UniqueId { self.id }
    
}

// ---- Area ---------------------------------------------------------------------------------------

pub struct Area {
    id: UniqueId,
}

impl Area {
    pub fn new(slug: String) -> Area {
        Area{
            id: super::id::next_id(),
        }    
    }
}

impl MapElement for Area {
    /// Id getter.    
    fn id(&self) -> UniqueId { self.id }
    
    fn bounding_box(&self) -> GeoBox {
        GeoBox::new(Location::new(0.0, 0.0), Location::new(0.0, 0.0)) // TODO
    }
}

// ---- test ---------------------------------------------------------------------------------------

#[test]
fn test_path() {
}

