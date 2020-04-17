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

pub mod distances;

#[cfg(test)]
mod tests {
    use crate::distances::Distances;
    use crate::distances::DistanceMeasure;

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
        assert!(vec1.euclidean_distance(&vec2).is_nan());
    }

    #[test]
    fn test_get_euclidean_distance_weighted()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.euclidean_distance_weighted(
                &vec2, &weights), 2.23606797749979);
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
        assert!(vec1.cosine_distance(&vec2).is_nan());
    }
    
    #[test]
    fn test_get_cosine_distance_weighted()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.cosine_distance_weighted(
                &vec2, &weights), 0.1339745962155614);
    }
    
    #[test]
    fn test_get_manhattan_distance() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.manhattan_distance(&vec2), 6.0);
    }

    #[test]
    fn test_get_manhattan_distance_different_vector_size() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0];
        assert!(vec1.manhattan_distance(&vec2).is_nan());
    }

    #[test]
    fn test_get_manhattan_distance_weighted()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.manhattan_distance_weighted(
                &vec2, &weights), 3.0);
    }

    #[test]
    fn test_get_distance_euclidean(){
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.distance(&vec2, DistanceMeasure::Euclidean), 
                   3.1622776601683795);
    }

    #[test]
    fn test_get_distance_weighted_euclidean()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.distance_weighted(&vec2, &weights,
                                          DistanceMeasure::Euclidean), 
                   2.23606797749979);
    }
    
    #[test]
    fn test_get_distance_cosine() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.distance(&vec2, DistanceMeasure::Cosine), 
                   0.16666666666666663);
    }

    #[test]
    fn test_get_distance_weighted_cosine()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.distance_weighted(
                &vec2, &weights, DistanceMeasure::Cosine), 
            0.1339745962155614);
    }
    
    #[test]
    fn test_get_distance_manhattan() {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        assert_eq!(vec1.distance(&vec2, DistanceMeasure::Manhattan), 
                   6.0);
    }

    #[test]
    fn test_get_distance_weighted_manhattan()  {
        let vec1 : Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 : Vec<f64> = vec![3.0, 1.0, 4.0, 2.0];
        let weights : Vec<f64> = vec![0.2, 0.4, 0.6, 0.8];
        assert_eq!(vec1.distance_weighted(
                &vec2, &weights, DistanceMeasure::Manhattan), 
            3.0);
    }
}
