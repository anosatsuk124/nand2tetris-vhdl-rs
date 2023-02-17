use icarus_verilog_testbench::IVerilogTest;
use std::path::PathBuf;

mod utils;

fn and(a: &str, b: &str) -> String {
    let test_top = format!(
        "
module AndTest;
   reg a, b;
   wire c;
   And and1(a, b, c);
   initial begin
      a = {a};
      b = {b};
      #1;
      $display(\"%b\", c);
    end
endmodule
"
    );
    let top_name = "AndTest";
    let dst = utils::create_test_top(test_top, top_name);
    let verilog_dir = PathBuf::from(utils::VERILOG_DIR);
    let boolean_dir = verilog_dir.join("boolean_gate");
    let test = IVerilogTest::builder()
        .top(top_name)
        .include(boolean_dir.clone().to_str().unwrap())
        .paths(vec![boolean_dir.clone().join("and.sv")])
        .path(dst)
        .build();
    let test = test.test(&PathBuf::from(env!("OUT_DIR")).join("nand_test"));
    test.unwrap().trim_end().to_string()
}

#[test]
fn and_test() {
    let test_file = PathBuf::from(utils::TEST_DATA_DIR).join("01/And.cmp");
    for test in utils::parse_cmp_file(test_file) {
        assert_eq!(
            and(test.get("a").unwrap(), test.get("b").unwrap()),
            test.get("out").unwrap().to_owned()
        );
    }
}
