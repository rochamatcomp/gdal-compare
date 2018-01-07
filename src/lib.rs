extern crate gdal;

use std::path::Path;
use gdal::raster::{Dataset, RasterBand};
use gdal::raster::dataset::GeoTransform;
use gdal::spatial_ref::SpatialRef;

fn get_dataset(filename: &str) -> Dataset{
    let path = Path::new(filename);
    Dataset::open(path).unwrap()
}

/// Compare the geotransform.
/// # Arguments
/// * `golden` - Georeferencing transform coefficients of the golden template data.
/// * `new` - Georeferencing transform coefficients of the new data.
pub fn compare_geo_transform(golden: GeoTransform, new: GeoTransform) -> bool{
    golden == new
}

/// Compare the projection.
/// # Arguments
/// * `golden` - Projection of the golden template data.
/// * `new` - Projection of the new data.
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

/// Compare the band
/// # Arguments
/// * `golden` - RasterBand of the golden template data.
/// * `new` - RasterBand of the new data.
/// * `band` - RasterBand number.
pub fn compare_band(golden: RasterBand, new: RasterBand, band: isize) -> bool{
    let golden_type = golden.band_type();
    let new_type = new.band_type();
    
    if golden_type != new_type{
        println!("Band {} pixel types differ.", band);
        println!("  Golden: {:?}", golden_type);
        println!("  New:    {:?}", new_type);
        return false;
    }

    let golden_blocks = golden.get_block_size();
    let new_blocks = new.get_block_size();

    println!("  Golden: {:?}", golden_blocks);
    println!("  New:    {:?}", new_blocks);
    
    if golden_blocks != new_blocks{
        println!("Band {} pixel blocks mismatch.", band);
        println!("  Golden: {:?}", golden_blocks);
        println!("  New:    {:?}", new_blocks);
        return false;
    }

    //TODO: nodata, statistics, metadata, color interpretation, checksum, overview, mask.
    true
}

/// Compare the all bands
/// # Arguments
/// * `golden` - Dataset of the golden template data.
/// * `new` - Dataset of the new data.
pub fn compare_bands(golden: Dataset, new: Dataset) -> bool{
    // Number of raster bands on these datasets
    let golden_count: isize = golden.count();
    let new_count: isize = new.count();
    
    if golden_count != new_count{
        println!("Bands count mismatch (golden: {}, new: {})", golden_count, new_count);
        return false;
    }

    // Number of pixels(x, y) on these datasets
    let golden_pixels: (usize, usize) = golden.size();
    let new_pixels: (usize, usize) = new.size();
    
    if golden_pixels != new_pixels{
        println!("Band pixels/sizes mismatch (golden: {:?}, new: {:?})", golden_pixels, new_pixels);
        return false;
    }

    let mut golden_band: RasterBand;
    let mut new_band: RasterBand;
    
    for band in 1..(golden_count+1){        
        golden_band = golden.rasterband(band).unwrap();
        new_band = new.rasterband(band).unwrap();
        
        if compare_band(golden_band, new_band, band) == false{
            return false;
        }
    }
    true
}

/*
    
/// Compare 
/// # Arguments
/// * `golden` -  of the golden template data.
/// * `new` -  of the new data.
pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

pub fn compare_(golden: String, new: String) -> bool{
    golden == new
}

*/

#[cfg(test)]
mod tests_geotransform {

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
}

#[cfg(test)]
mod tests_projection {

    use super::*;

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
}

#[cfg(test)]
mod tests_bands {

    use super::*;

    #[test]
    fn test_bands_count_equals() {
        let golden: Dataset = get_dataset("data/golden.tif");
        let new: Dataset = get_dataset("data/new_bands_equals.tif");
        
        assert_eq!(true, compare_bands(golden, new));
    }

    #[test]
    fn test_bands_count_diff() {
        let golden: Dataset = get_dataset("data/golden.tif");
        let new: Dataset = get_dataset("data/new_bands_diff.tif");
        
        assert_eq!(false, compare_bands(golden, new));
    }

    #[test]
    fn test_band_pixels_equals() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_pixels_equals.asc");
        
        assert_eq!(true, compare_bands(golden, new));
    }

    #[test]
    fn test_band_pixels_diff() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_pixels_diff.asc");
        
        assert_eq!(false, compare_bands(golden, new));
    }

    #[test]
    fn test_band_type_equal() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_type_equal.asc");
        
        assert_eq!(true, compare_bands(golden, new));
    }

    #[test]
    fn test_band_type_diff() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_type_diff.asc");
        
        assert_eq!(false, compare_bands(golden, new));
    }

    #[test]
    fn test_band_blocks_equals() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_blocks_equals.asc");
        
        assert_eq!(true, compare_bands(golden, new));
    }

    #[test]
    fn test_band_blocks_diff() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_blocks_diff.asc");
        
        assert_eq!(false, compare_bands(golden, new));
    }

    
    #[test]
    fn test_band_blocks_single() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_blocks_single.asc");
        
        assert_eq!(true, compare_bands(golden, new));
    }
    
    #[test]
    fn test_band_blocks_double() {
        let golden: Dataset = get_dataset("data/golden.asc");
        let new: Dataset = get_dataset("data/new_blocks_double.asc");
        
        assert_eq!(false, compare_bands(golden, new));
    }
}


/*
#[cfg(test)]
mod tests_metada {

    use super::*;

    #[test]
    fn test_metadata() {
        assert_eq!(true, compare_metadata(golden, new));
    }
}

#[cfg(test)]
mod tests_nodata {

    use super::*;

    #[test]
    fn test_nodata() {       
        assert_eq!(true, compare_nodata(golden, new));
    }
}

#[cfg(test)]
mod tests_datasets {

    use super::*;

    #[test]
    fn test_datasets() {
        assert_eq!(true, compare_datasets(golden, new));
    }
}
*/
