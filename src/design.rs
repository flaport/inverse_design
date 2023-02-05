use super::status::Status;
use super::sparse::new_sparse;

pub fn test_design() {
    let design = new_sparse(5, 5, Status::Unassigned);
    println!("{design:?}");
}

