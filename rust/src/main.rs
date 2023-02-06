use inverse_design_rs::profiling::print_profiler_summary;
use inverse_design_rs::visualization::test_visualization;
use inverse_design_rs::design::test_design;
// use inverse_design_rs::brushes::test_brushes;
// use inverse_design_rs::design::test_design;
// use inverse_design_rs::array::test_array;

fn main() {
    test_visualization();
    //test_brushes();
    test_design();
    print_profiler_summary();
}
