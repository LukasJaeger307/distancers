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

mod distances;

#[cfg(test)]
mod tests {
    use crate::distances::Distances;

    #[test]
    fn test_get_euclidean_distance() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.euclidean_distance(&vec2), 3.1622776601683795);
    }

    #[test]
    fn test_get_euclidean_distance_different_vector_size() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0];
        assert_eq!(vec1.euclidean_distance(&vec2), -1.0);
    }
    
    #[test]
    fn test_get_cosine_distance() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.cosine_distance(&vec2), 0.16666666666666663);
    }

    #[test]
    fn test_get_cosine_distance_different_vector_size() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0];
        assert_eq!(vec1.cosine_distance(&vec2), -1.0);
    }

}
