use crate::bit_utils::to_6bit_binary;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinate {
    pub i: usize,
    pub j: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "coord:({},{})", self.i, self.j,)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CoordinateAndNet {
    pub coordinate: Coordinate,
    pub net: u32,
}

impl fmt::Display for CoordinateAndNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{}):{}",
            self.coordinate.i,
            self.coordinate.j,
            to_6bit_binary(&self.net)
        )
    }
}

#[derive(Debug)]
pub struct CataloguedNet {
    pub hidden: HashSet<CoordinateAndNet>,
    pub visible: Vec<CoordinateAndNet>,
}

//
impl Coordinate {
    pub fn new() -> Self {
        Self { i: 0, j: 0 }
    }
}

// Implementing Hash for Coordinate
impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

pub fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        if u < i.wrapping_abs() as u32 as usize {
            return None;
        }
        let r = u - i.wrapping_abs() as u32 as usize;
        return Some(r);
    }
    let r = u + i as usize;
    return Some(r);
}

impl CoordinateAndNet {
    pub fn new(coordinate: Coordinate, net: u32) -> CoordinateAndNet {
        return CoordinateAndNet { coordinate, net };
    }
}

impl Eq for CoordinateAndNet {}

impl PartialEq for CoordinateAndNet {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate && self.net == other.net
    }
}

impl Hash for CoordinateAndNet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
        self.net.hash(state);
    }
}

pub struct ComplexNet {
    pub hidden_neighbours: HashSet<CoordinateAndNet>, // The nets that are to be replaced, i.e. hidden.
    pub complex_net: CoordinateAndNet, // The complex net that will replace more than one net in one go.
}

impl ComplexNet {
    pub fn new() -> ComplexNet {
        return ComplexNet {
            hidden_neighbours: HashSet::new(),
            complex_net: CoordinateAndNet::new(Coordinate { i: 0, j: 0 }, 0),
        };
    }
}

impl CataloguedNet {
    pub fn new() -> CataloguedNet {
        return CataloguedNet {
            hidden: HashSet::new(),
            visible: vec![],
        };
    }

    pub fn add(&mut self, complex_net: ComplexNet) {
        self.hidden.extend(&complex_net.hidden_neighbours.clone());
        self.visible.push(complex_net.complex_net);
    }

    pub fn extend(&mut self, catalogued_net: CataloguedNet) {
        self.hidden.extend(&catalogued_net.hidden);
        self.visible.extend(catalogued_net.visible);
    }
}

impl fmt::Display for CataloguedNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "visible: {:?} ,hidden: {:?}", self.visible, self.hidden,)
    }
}
