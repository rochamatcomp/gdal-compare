extern crate gdal;

use std::path::Path;
/*
#[macro_use]
extern crate ndarray;

use std::collections::HashMap;

use gdal::raster::{Driver, Dataset, Buffer};
use ndarray::Array2;
*/
use gdal::raster::{Dataset};
use gdal::raster::dataset::GeoTransform;
use gdal::spatial_ref::SpatialRef;

fn get_dataset(filename: &str) -> Dataset{
    let path = Path::new(filename);
    Dataset::open(path).unwrap()
}

/// Compare the geotransform.
pub fn compare_geo_transform(golden: GeoTransform, new: GeoTransform) -> bool{
    golden == new
}

/// Compare the projection.
pub fn compare_projection(golden: String, new: String) -> bool{
    if golden == new{
        return true;
    }

    // Golden projection as WKT and new projection as ESRI formats
    let spatial_ref1 = SpatialRef::from_wkt(&golden).unwrap();
    let spatial_ref2 = SpatialRef::from_esri(&new).unwrap();

    // Golden projection as ESRI and new projection as WKT formats
    let spatial_ref3 = SpatialRef::from_esri(&golden).unwrap();
    let spatial_ref4 = SpatialRef::from_wkt(&new).unwrap();

    (spatial_ref1 == spatial_ref2) & (spatial_ref3 == spatial_ref4)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_geotransform_eq() {
        let golden: GeoTransform = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let new: GeoTransform = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        assert_eq!(true, compare_geo_transform(golden, new));
    }
    
    #[test]
    fn test_geotransform_diff() {
        let golden: GeoTransform = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let new: GeoTransform = [1.0, 2.0, 3.0, 4.0, 5.0, 5.0];
        
        assert_eq!(false, compare_geo_transform(golden, new));
    }

    #[test]
    fn test_geotransform_dataset_eq() {     
        let dataset1: Dataset = get_dataset("data/golden.asc");
        let dataset2: Dataset = get_dataset("data/new_equal.asc");
        let golden: GeoTransform = dataset1.geo_transform().unwrap();
        let new: GeoTransform = dataset2.geo_transform().unwrap();

        assert_eq!(true, compare_geo_transform(golden, new));
    }

    #[test]
    fn test_geotransform_dataset_diff() {
        let dataset1: Dataset = get_dataset("data/golden.asc");
        let dataset2: Dataset = get_dataset("data/new_diff.asc");
        let golden: GeoTransform = dataset1.geo_transform().unwrap();
        let new: GeoTransform = dataset2.geo_transform().unwrap();

        assert_eq!(false, compare_geo_transform(golden, new));
    }

    #[test]
    fn test_projection_eq() {
        let golden: String = String::from("GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]]");
        
        let new: String = String::from("GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]]");

        assert_eq!(true, compare_projection(golden, new));
    }
    
    #[test]
    fn test_projection_diff() {
        let golden: String = String::from("GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]]");
        
        let new: String = String::from("GEOGCS[\"SIRGAS 2000\",DATUM[\"Sistema_de_Referencia_Geocentrico_para_America_del_Sur_2000\",SPHEROID[\"GRS 1980\",6378137,298.257222101,AUTHORITY[\"EPSG\",\"7019\"]],TOWGS84[0,0,0,0,0,0,0],AUTHORITY[\"EPSG\",\"6674\"]],PRIMEM[\"Greenwich\",0, AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4674\"]]");
        
        assert_eq!(false, compare_projection(golden, new));
    }

    #[test]
    fn test_projection_dataset_eq() {     
        let dataset1: Dataset = get_dataset("data/golden.asc");
        let dataset2: Dataset = get_dataset("data/new_equal.asc");
        let golden: String = dataset1.projection();
        let new: String = dataset2.projection();

        assert_eq!(true, compare_projection(golden, new));
    }

    #[test]
    fn test_projection_dataset_diff() {
        let dataset1: Dataset = get_dataset("data/golden.asc");
        let dataset2: Dataset = get_dataset("data/new_diff.asc");
        let golden: String = dataset1.projection();
        let new: String = dataset2.projection();

        assert_eq!(false, compare_projection(golden, new));
    }

    #[test]
    fn test_projection_spatial_ref_eq() {     
        let dataset1: Dataset = get_dataset("data/golden.asc");
        let dataset2: Dataset = get_dataset("data/new_spatial_ref_equal.asc");
        let golden: String = dataset1.projection();
        let new: String = dataset2.projection();

        assert_eq!(true, compare_projection(golden, new));

        let new: String = dataset1.projection();
        let golden: String = dataset2.projection();
        
        assert_eq!(true, compare_projection(new, golden));
    }

    #[test]
    fn test_metadata() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_nodata() {
        assert_eq!(true, false);
    }
    
    #[test]
    fn test_bands() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_srs() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_dimensions() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_datasets() {
        assert_eq!(true, false);
    }
}
