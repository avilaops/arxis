// Placeholder para loader HDF5
// TODO: Implementar quando feature "scientific-io" for habilitada

use crate::error::Result;

/// Carrega dataset de arquivo HDF5
pub fn load_hdf5_dataset(_path: &str, _dataset_name: &str) -> Result<Vec<f32>> {
    unimplemented!("HDF5 loading requires 'scientific-io' feature")
}
