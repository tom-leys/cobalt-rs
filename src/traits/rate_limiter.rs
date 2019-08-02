// Copyright (c) 2015-2017 Ivo Wetzel

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// STD Dependencies -----------------------------------------------------------
use std::fmt;


// Internal Dependencies ------------------------------------------------------
use super::super::Config;


/// Trait describing a network congestion avoidance algorithm.
pub trait RateLimiter {

    /// Method that constructs a new rate limiter using the provided configuration.
    fn new(Config) -> Self where Self: Sized;

    /// Method implementing a congestion avoidance algorithm based on round
    /// trip time and packet loss.
    fn update(&mut self, rtt: u32, packet_loss: f32);

    /// Method that should return `true` in case the connection is currently
    /// considered congested and should reduce the number of packets it sends
    /// per second.
    fn congested(&self) -> bool;

    /// Method that returns whether a connection should be currently sending
    /// packets or not.
    fn should_send(&self) -> bool;

    /// Method that resets any internal state of the rate limiter.
    fn reset(&mut self);

}

impl fmt::Debug for dyn RateLimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RateLimiter")
    }
}

