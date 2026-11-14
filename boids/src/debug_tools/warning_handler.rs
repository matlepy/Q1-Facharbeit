use crate::boid::Boid;
use crate::config::boundary::*;
use log::{info, warn};

//Handler for debug info and warnings, to avoid cluttering the boid code with debug info and warnings.
pub struct WarningHandler {
    global_boundary: Boundary,
    info: bool,
}

impl WarningHandler {
    pub fn new(boundary: Boundary, info: bool) -> Self {
        WarningHandler {
            global_boundary: boundary,
            info,
        }
    }

    pub fn check(&self, boid: &Boid) -> bool {
        match &self.global_boundary {
            Boundary::Cube(c) => {
                if c.is_inside(boid.position) && boid.inside_boundary == false {
                    //info and warning only when boid changes from inside to outside or vice versa
                    if self.info {
                        info!("boid {} is in boundary", boid.id);
                    }
                    return true;
                } else if !c.is_inside(boid.position) && boid.inside_boundary == true {
                    warn!("boid {} is not in boundary", boid.id);
                    return false;
                } else {
                    return boid.inside_boundary;
                }
            }
            Boundary::Sphere(s) => {
                if s.is_inside(boid.position) && boid.inside_boundary == false {
                    if self.info {
                        info!("boid {} is in boundary", boid.id);
                    }
                    return true;
                } else if !s.is_inside(boid.position) && boid.inside_boundary == true {
                    warn!("boid {} is not in boundary", boid.id);
                    return false;
                } else {
                    return boid.inside_boundary;
                }
            }
        }
    }
}
