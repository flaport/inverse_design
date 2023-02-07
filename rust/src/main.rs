use inverse_design_rs::visualization::test_visualization;

// use inverse_design_rs::array::test_array;
// use inverse_design_rs::brushes::test_brushes;
// use inverse_design_rs::design::test_design;
use inverse_design_rs::debug::print_profiler_summary;
use inverse_design_rs::generator::test_generator;

fn main() {
    test_visualization();
    //test_array();
    //test_brushes();
    // test_design();
    test_generator();
    print_profiler_summary();
}
