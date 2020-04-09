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

pub trait Distances<T> {
    fn euclidean_distance(&self, other: &T) -> f64;
    fn cosine_distance(&self, other: &T) -> f64;
}

impl Distances<Vec<f64>> for Vec<f64> {
    fn euclidean_distance(&self, other : &Vec<f64>) -> f64 {
        if self.len() != other.len() {
            -1.0
        } else {
            let mut distance : f64 = 0.0;
            for i in 0..self.len() {
                distance += (self[i] - other[i]).powf(2.0);
            }
            distance.sqrt()
        }
    }

    fn cosine_distance(&self, other : &Vec<f64>) -> f64 {
        if self.len() != other.len() {
            -1.0
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
}
