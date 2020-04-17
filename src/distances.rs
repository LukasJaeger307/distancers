/*
 *  (c) 2020, Lukas Jäger
 *
 *  This file is part of distancers.
 *
 *  distancers is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  distancers is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public
 *  License along with distancers.  If not, see
 *  <https://www.gnu.org/licenses/>.
 */

pub enum DistanceMeasure {
    Euclidean,
    Cosine,
    Manhattan,
}

pub trait Distances<T> {
    fn distance(&self, other : &T, measure : DistanceMeasure) -> f64;
    fn distance_weighted(&self, other : &T, weights : &T, 
                         measure : DistanceMeasure) -> f64;
    fn euclidean_distance(&self, other : &T) -> f64;
    fn euclidean_distance_weighted(&self, other : &T, weights : &T) -> f64;
    fn cosine_distance(&self, other : &T) -> f64;
    fn cosine_distance_weighted(&self, other : &T, weights : &T) -> f64;
    fn manhattan_distance(&self, other : &T) -> f64;
    fn manhattan_distance_weighted(&self, other : &T, weights : &T) -> f64;
}

impl Distances<Vec<f64>> for Vec<f64> {
    
    fn distance(&self, other : &Vec<f64>, measure : DistanceMeasure) -> f64 {
        match measure {
            DistanceMeasure::Euclidean => {
                self.euclidean_distance(other)
            },
            DistanceMeasure::Cosine => {
                self.cosine_distance(other)
            },
            DistanceMeasure::Manhattan => {
                self.manhattan_distance(other)
            }
        }
    }

    fn distance_weighted(&self, other : &Vec<f64>, weights : &Vec<f64>, 
                         measure : DistanceMeasure) -> f64 {
        match measure {
            DistanceMeasure::Euclidean => {
                self.euclidean_distance_weighted(other, weights)
            },
            DistanceMeasure::Cosine => {
                self.cosine_distance_weighted(other, weights)
            }
            DistanceMeasure::Manhattan => {
                self.manhattan_distance_weighted(other, weights)
            }
        }
    }
    
    fn euclidean_distance(&self, other : &Vec<f64>) -> f64 {
        if self.len() != other.len() {
            std::f64::NAN
        } else {
            let mut distance : f64 = 0.0;
            for i in 0..self.len() {
                distance += (self[i] - other[i]).powf(2.0);
            }
            distance.sqrt()
        }
    }

    fn euclidean_distance_weighted(&self, other : &Vec<f64>, 
                                   weights : &Vec<f64>) -> f64 {
        if self.len() != other.len() || self.len() != weights.len() {
            std::f64::NAN
        } else {
            let mut distance : f64 = 0.0;
            for i in 0..self.len() {
                distance += weights[i] * (self[i] - other[i]).powf(2.0);
            }
            distance.sqrt()
        }
    }

    fn cosine_distance(&self, other : &Vec<f64>) -> f64 {
        if self.len() != other.len() {
            std::f64::NAN
        } else {
            let mut dividend : f64 = 0.0;
            let mut left_divisor : f64 = 0.0;
            let mut right_divisor : f64 = 0.0;
            for i in 0..self.len() {
                dividend += self[i] * other[i];
                left_divisor += self[i].powf(2.0);
                right_divisor += other[i].powf(2.0);
            }
            let divisor = left_divisor.sqrt() * right_divisor.sqrt();
            1.0 - (dividend / divisor)
        }
    }
    
    fn cosine_distance_weighted(&self, other : &Vec<f64>, 
                                   weights : &Vec<f64>) -> f64 {
        if self.len() != other.len() || self.len() != weights.len() {
            std::f64::NAN
        } else {
            let mut dividend : f64 = 0.0;
            let mut left_divisor : f64 = 0.0;
            let mut right_divisor : f64 = 0.0;
            for i in 0..self.len() {
                dividend += weights[i] * self[i] * other[i];
                left_divisor += weights[i] * self[i].powf(2.0);
                right_divisor += weights[i] * other[i].powf(2.0);
            }
            let divisor = left_divisor.sqrt() * right_divisor.sqrt();
            1.0 - (dividend / divisor)
        }
    }
    
    fn manhattan_distance(&self, other : &Vec<f64>) -> f64 {
        if self.len() != other.len() {
            std::f64::NAN
        } else {
            let mut distance : f64 = 0.0;
            for i in 0..self.len() {
                distance += (self[i] - other[i]).abs();
            }
            distance
        }
    }

    fn manhattan_distance_weighted(&self, other : &Vec<f64>, weights : &Vec<f64>) 
        -> f64 {
        if self.len() != other.len() {
            std::f64::NAN
        } else {
            let mut distance : f64 = 0.0;
            for i in 0..self.len() {
                distance += weights[i] * (self[i] - other[i]).abs();
            }
            distance
        }
    }
}
