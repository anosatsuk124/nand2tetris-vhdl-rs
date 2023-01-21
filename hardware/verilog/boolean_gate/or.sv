module Or(input a, b, output out);
   wire c;
   Not not1(a, c);
   wire d;
   Not not2(b, d);

   wire e;
   Nand nand1(c, d, e);

   assign out = e;
endmodule
