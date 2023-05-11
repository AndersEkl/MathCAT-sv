// UEB tests for the basic mathml tags
// Initial tests are from BANA guidelines, mostly about initial chars for code switching
//   http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
// These tests start with "bana_"
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

#[test]
fn bana_2_1() {
    let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>&#xD7;</mo><mn>2</mn><mo>&#xD7;</mo><mn>3</mn>
                <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉");
}

#[test]
fn bana_5_1() {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋");
}

#[test]
fn bana_5_2() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup><mo>=</mo><mi>C</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠰⠠⠉");
}

#[test]
fn bana_5_3() {
    let expr = "<math><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mfrac><mi>c</mi><mi>d</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠨⠌⠃⠾⠐⠖⠷⠉⠨⠌⠙⠾");
}

#[test]
fn bana_5_4() {
    let expr = "<math><msup><mi>a</mi><mi>n</mi></msup><mo>&#xD7;</mo><msup><mi>a</mi><mi>m</mi></msup><mo>=</mo>
                    <msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠁⠔⠣⠝⠐⠖⠍⠜⠰⠄");
}

#[test]
fn bana_5_5() {
    let expr = "<math><msub><mi>log</mi><mi>x</mi></msub><mi>y</mi></math>";
    // BANA example contradicts GTM 9.2 that says don't use a space after a function name if there is an intervening indicator.
    // Corrected: removed the space and the G1 indicator needed if a space were inserted
    test_braille("UEB", expr, "⠰⠰⠇⠕⠛⠢⠭⠽");
}

#[test]
fn bana_5a_1() {
    let expr = "<math><msup><mn>100</mn><mo>&#xB0;</mo></msup><mi>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋");
}

#[test]
fn bana_5a_1_baseline() {
    let expr = "<math><mn>100</mn><mo>&#xB0;</mo><mi class='MathML-unit'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋");
}

#[test]
fn bana_5a_2() {
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mi class='MathML-unit'>km</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃");
}

#[test]
fn bana_5a_2_mtext() {
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mtext class='MathML-unit'>km</mtext><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃");
}

#[test]
fn bana_5a_3() {
    let expr = "<math><mn>6</mn><mo>&#xA0;</mo><mi class='MathML-unit'>m</mi><mo>&#xA0;</mo>
            <msup><mi class='MathML-unit'>s</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠰⠍⠀⠰⠰⠎⠔⠣⠐⠤⠼⠁⠜");
}

#[test]
fn bana_6_1() {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mi>z</mi>
                        <mo>=</mo><msup><mi>t</mi><mn>2</mn></msup><mo>.</mo></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠐⠖⠽⠀⠐⠶⠀⠵⠀⠐⠶⠀⠞⠔⠼⠃⠲⠰⠄");
}

#[test]
fn cap_1_6_1() {
    let expr = "<math><mi>ABCD</mi></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠉⠙");
}

#[test]
fn cap_1_6_1_separate() {
    let expr = "<math>
        <mi mathvariant='normal'>A</mi>
        <mi mathvariant='normal'>B</mi>
        <mi mathvariant='normal'>C</mi>
        <mi mathvariant='normal'>D</mi></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠉⠙");
}

#[test]
fn cap_1_6_2() {
    let expr = "<math><mi>V</mi><mo>=</mo><mi>I</mi><mi>R</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠧⠀⠐⠶⠀⠠⠠⠊⠗");
}

#[test]
fn cap_1_6_4() {
    let expr = "<math><mi>A</mi><msup><mi>B</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠠⠠⠁⠃⠰⠔⠼⠃");
}

#[test]
fn grade1_1_7_1() {
    let expr = "<math><mn>3</mn><mo>-</mo><mn>2</mn>
                            <mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac ><mo>=</mo></math>";
    // removed the spaces around the '-' from the braille -- they typically wouldn't be used
    test_braille("UEB", expr, "⠼⠉⠐⠤⠼⠃⠼⠁⠌⠃⠀⠐⠶");
}

#[test]
fn grade1_1_7_2() {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi><mo>+</mo><mn>4</mn><mi>c</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠭⠐⠖⠼⠙⠰⠉");
}

#[test]
fn grade1_1_7_3_1() {
    let expr = "<math>
        <mn>3</mn><mi>x</mi><mo>&#x2212;</mo><mn>4</mn><mi>y</mi><mo>+</mo>
        <msup><mi>y</mi><mn>2</mn></msup>
        <mo>=</mo>
        <msup><mi>x</mi><mn>2</mn></msup>
    </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠼⠉⠭⠐⠤⠼⠙⠽⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠭⠔⠼⠃⠰⠄");
}

#[test]
fn grade1_1_7_3_2() {
    let expr = "<math> <mfrac>
    <mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>x</mi></mrow>
    <mrow><mn>1</mn><mo>+</mo><msup><mi>x</mi><mn>2</mn></msup></mrow>
    </mfrac><mo>=</mo><mn>1</mn>
    </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠷⠭⠔⠼⠃⠐⠖⠼⠃⠭⠨⠌⠼⠁⠐⠖⠭⠔⠼⠃⠾⠀⠐⠶⠀⠼⠁⠰⠄");
}

#[test]
fn grade1_1_7_4() {
    let expr = "<math><msqrt>
            <mo>(</mo><mi>y</mi><mo>&#x2212;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo>
        </msqrt></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠐⠣⠽⠐⠤⠭⠔⠼⠃⠐⠜⠬");
}

#[test]
fn number_2_1_2() {
    let expr = "<math><mn>3,000</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠂⠚⠚⠚");
}

#[test]
fn number_2_1_3() {
    let expr = "<math><mn>5 000 000</mn></math>";
    test_braille("UEB", expr, "⠼⠑⠐⠚⠚⠚⠐⠚⠚⠚");
}

#[test]
fn number_2_2_1() {
    let expr = "<math><mn>8.93</mn></math>";
    test_braille("UEB", expr, "⠼⠓⠲⠊⠉");
}

#[test]
fn number_2_2_2() {
    let expr = "<math><mn>0.7</mn></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠛");
}

#[test]
fn number_2_2_3() {
    let expr = "<math><mn>.7</mn></math>";
    test_braille("UEB", expr, "⠼⠲⠛");
}

#[test]
fn time_2_4_1() {
    let expr = "<math><mn>5</mn><mo>:</mo><mn>30</mn><mo>&#xA0;</mo><mtext>pm</mtext></math>";
    test_braille("UEB", expr, "⠼⠑⠒⠼⠉⠚⠀⠏⠍");
}

#[test]
fn roman_numeral_2_6_1() {
    let expr = " <math><mi mathvariant='normal'>I</mi><mo>,</mo>
        <mo>&#xA0;</mo><mi>II</mi>
        <mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi mathvariant='normal'>V</mi></math>";
    test_braille("UEB", expr, "⠠⠊⠂⠀⠠⠠⠊⠊⠀⠯⠀⠰⠠⠧");
}

#[test]
fn roman_numeral_2_6_2() {
    let expr = " <math><mi mathvariant='normal'>i</mi><mo>,</mo>
        <mo>&#xA0;</mo><mi>vi</mi>
        <mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi mathvariant='normal'>x</mi></math>";
    test_braille("UEB", expr, "⠊⠂⠀⠧⠊⠀⠯⠀⠰⠭");
}

#[test]
fn roman_numeral_2_6_3() {
    let expr = "<math><mn>CD</mn></math>";
    test_braille("UEB", expr, "⠰⠠⠠⠉⠙");
}

#[test]
fn bold_2_7_1() {
    let expr = "<math><mn>67𝟖45</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠆⠼⠓⠙⠑");
}

#[test]
fn bold_2_7_2() {
    let expr = "<math><mn>67</mn><mn mathvariant='bold'>845</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠂⠼⠓⠙⠑");
}

#[test]
fn bold_2_7_3() {
    let expr = "<math><mn>67</mn><mn mathvariant='bold'>84</mn><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠂⠼⠓⠙⠘⠄⠼⠑");
}

#[test]
fn signs_2_10_2() {
    let expr = "<math><mo>$</mo><mn>0.30</mn><mo>,</mo><mo>&#xA0;</mo>
                <mn>30</mn><mi mathvariant='normal'>c</mi><mo>&#xA0;</mo>
                <mtext>or</mtext><mo>&#xA0;</mo><mn>30</mn><mo>&#xA2;</mo></math>";
    test_braille("UEB", expr, "⠈⠎⠼⠚⠲⠉⠚⠂⠀⠼⠉⠚⠰⠉⠀⠕⠗⠀⠼⠉⠚⠈⠉");
}

#[test]
fn signs_2_10_5() {
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi>ft</mi><mo>&#xA0;</mo><mn>6</mn><mo>&#xA0;</mo><mi>in</mi>
        <mo>&#xA0;</mo><mtext>or</mtext><mo>&#xA0;</mo>
        <mn>1</mn><mo>&#x2032;</mo><mo>&#xA0;</mo><mn>6</mn><mo>&#x2032;</mo><mo>&#x2032;</mo></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠋⠞⠀⠼⠋⠀⠔⠀⠕⠗⠀⠼⠁⠶⠀⠼⠋⠶⠶");
}

#[test]
fn signs_2_10_8() {
    let expr = "<math><mn>0</mn><mo>&#xB0;</mo><mi mathvariant='normal'>C</mi><mo>&#xA0;</mo><mtext>or</mtext>
        <mo>&#xA0;</mo><mn>32</mn><mo>&#xB0;</mo><mi mathvariant='normal'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠚⠘⠚⠠⠉⠀⠕⠗⠀⠼⠉⠃⠘⠚⠠⠋");
}

#[test]
fn signs_2_10_16() {
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi mathvariant='normal'>&#xC5;</mi><mo>=</mo>
        <mfrac><mn>1</mn><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></mfrac><mo>&#xA0;</mo>
        <mi mathvariant='normal'>&#x3BC;</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠠⠘⠫⠁⠀⠐⠶⠀⠼⠁⠌⠁⠚⠂⠚⠚⠚⠀⠨⠍");
}

#[test]
fn expr_3_1_1() {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>5</mn><mo>=</mo><mn>8</mn></math>";
    // correct not to use extra spacing
    test_braille("UEB", expr, "⠼⠉⠐⠖⠼⠑⠀⠐⠶⠀⠼⠓");
}

#[test]
fn expr_3_1_2() {
    let expr = "<math><mn>8</mn><mo>-</mo><mn>5</mn><mo>=</mo><mn>3</mn></math>";
    // correct not to use extra spacing
    test_braille("UEB", expr, "⠼⠓⠐⠤⠼⠑⠀⠐⠶⠀⠼⠉");
}

#[test]
fn expr_3_1_3() {
    let expr = "<math><mn>3</mn><mo>&#xD7;</mo><mn>5</mn><mo>=</mo><mn>5</mn><mo>&#xD7;</mo><mn>3</mn><mo>=</mo><mn>15</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠐⠦⠼⠑⠀⠐⠶⠀⠼⠑⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠑");
}

#[test]
fn expr_3_1_6() {
    // example includes spaces, so does the MathML (from WIRIS)
    let expr = "<math><mn>5</mn><mo>.</mo><mn>72</mn><mo>&#xA0;</mo><mtext>m</mtext><mo>&#xF7;</mo><mn>10</mn><mo>=</mo>
                    <mn>57</mn><mo>.</mo><mn>2</mn><mo>&#xA0;</mo><mi>cm</mi></math>";
    test_braille("UEB", expr, "⠼⠑⠲⠛⠃⠀⠍⠐⠌⠼⠁⠚⠀⠐⠶⠀⠼⠑⠛⠲⠃⠀⠉⠍");
}

#[test]
fn expr_3_1_7() {
    let expr = "<math><mn>15</mn><mo>&#xB1;</mo><mn>0</mn><mo>.</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠁⠑⠸⠖⠼⠚⠲⠑");
}

#[test]
fn expr_3_1_8() {
    let expr = "<math><mi>Area</mi><mo>=</mo><mi>b</mi><mi>h</mi><mo>=</mo>
            <mn>5</mn><mo>&#xB7;</mo><mn>3</mn><mo>=</mo><mn>15</mn></math>";
    test_braille("UEB", expr, "⠠⠜⠑⠁⠀⠐⠶⠀⠃⠓⠀⠐⠶⠀⠼⠑⠐⠲⠼⠉⠀⠐⠶⠀⠼⠁⠑");
}

#[test]
fn expr_3_1_9_wiris() {
    let expr = "<math><mn>3</mn><mo>.</mo><mn>9</mn><mo>&#xD7;</mo><mn>4</mn><mo>.</mo><mn>1</mn><mo>&#x2243;</mo><mn>16</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠲⠊⠐⠦⠼⠙⠲⠁⠀⠸⠔⠀⠼⠁⠋");
}

#[test]
fn expr_3_1_9() {
    let expr = "<math><mn>3.9</mn><mo>&#xD7;</mo><mn>4.1</mn><mo>&#x2243;</mo><mn>16</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠲⠊⠐⠦⠼⠙⠲⠁⠀⠸⠔⠀⠼⠁⠋");
}

#[test]
fn expr_3_1_10() {
    let expr = "<math><mn>5</mn><mo>-</mo><mn>3</mn><mo>&#x2260;</mo><mn>3</mn><mo>-</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠑⠐⠤⠼⠉⠀⠐⠶⠈⠱⠀⠼⠉⠐⠤⠼⠑");
}

#[test]
fn ratio_3_1_11() {
    let expr = "<math><mn>1</mn><mo>:</mo><mn>200</mn></math>";
    test_braille("UEB", expr, "⠼⠁⠒⠼⠃⠚⠚");
}

#[test]
fn ratio_3_1_12() {
    let expr = "<math><mn>2</mn><mo>:</mo><mn>4</mn><mo>=</mo><mn>6</mn><mo>:</mo><mn>12</mn></math>";
    test_braille("UEB", expr, "⠼⠃⠒⠼⠙⠀⠐⠶⠀⠼⠋⠒⠼⠁⠃");
}

#[test]
fn alg_3_2_1_1() {
    let expr = "<math><mi>y</mi><mo>&#x221D;</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠸⠐⠶⠀⠰⠭");
}

#[test]
fn alg_3_2_1_2() {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>k</mi><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠅⠭");
}

#[test]
fn alg_3_2_2() {
    let expr = "<math><mn>0</mn><mo>&#x2264;</mo><mi>&#x3B8;</mi>
            <mo>&#x2264;</mo><mn>2</mn><mi mathvariant='normal'>&#x3C0;</mi></math>";
    test_braille("UEB", expr, "⠼⠚⠀⠸⠈⠣⠀⠨⠹⠀⠸⠈⠣⠀⠼⠃⠨⠏");
}

#[test]
fn alg_3_2_3() {
    let expr = "<math><mi>y</mi><mo>=</mo><mi>x</mi><mo>+</mo><mn>4</mn></math>";
    test_braille("UEB", expr, "⠰⠽⠀⠐⠶⠀⠭⠐⠖⠼⠙");
}


#[test]
fn alg_3_2_4() {
    let expr = "<math><mn>2</mn><mi>y</mi><mo>=</mo><mn>2</mn><mi>c</mi><mo>-</mo><mn>4</mn></math>";
    test_braille("UEB", expr, "⠼⠃⠽⠀⠐⠶⠀⠼⠃⠰⠉⠐⠤⠼⠙");
}

#[test]
fn alg_3_2_5() {
    let expr = "<math><mi>d</mi><mo>+</mo><mi>a</mi><mi>b</mi><mo>=</mo><mi>a</mi><mi>c</mi></math>";
    // Acceptable: GTM does not use a G1 start indicator: "⠙⠐⠖⠁⠃⠀⠐⠶⠀⠰⠁⠉"
    // However, BANA says use a word indicator if G1 not in first 3 cells (it is after the '='); use passage if >=2 whitespace
    // This seems like a poor choice in this case since there is only one G1 indicator, but that's the BANA guidance so...
    // Corrected to use passage indicator
    test_braille("UEB", expr, "⠰⠰⠰⠙⠐⠖⠁⠃⠀⠐⠶⠀⠁⠉⠰⠄");
}

#[test]
fn ratio_3_2_6() {
    // the difference from ratio_3_1_12 is this involves letters
    let expr = "<math><mi>x</mi><mo>:</mo><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠒⠽");
}

#[test]
fn example_3_4_1() {
    let expr = "<math><mo>-</mo><mn>4</mn><mtext>&#xA0;to&#xA0;</mtext><mo>+</mo><mn>5</mn></math>";
    test_braille("UEB", expr, "⠐⠤⠼⠙⠀⠞⠕⠀⠐⠖⠼⠑");
}

#[test]
fn example_3_4_2() {
    // removed some cruft from TeX output of {}^{-}2+{}^{+}3, but the basics are preserved
    let expr = "<math>
        <msup> <mrow/> <mo>&#x2212;</mo></msup>
        <mn>2</mn>
        <mo>+</mo>
        <msup> <mrow/> <mo>&#x2212;</mo></msup>
        <mn>3</mn>
    </math>";
    test_braille("UEB", expr, "⠰⠔⠐⠤⠼⠃⠐⠖⠔⠐⠤⠼⠉");
}

#[test]
fn fraction_6_1_1() {
    let expr = "<math><mfrac><mn>5</mn><mn>8</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠌⠓");
}

#[test]
fn fraction_6_1_2() {
    let expr = "<math><mfrac><mrow><mn>5</mn><mo>.</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>,</mo><mn>000</mn></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠲⠛⠌⠃⠂⠚⠚⠚");
}

#[test]
fn fraction_6_2_1() {
    let expr = "<math><mn>2</mn><mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠃⠼⠁⠌⠃");
}

#[test]
fn fraction_6_2_2() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mfrac bevelled='true'><mn>3</mn><mn>4</mn></mfrac>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_2_2_unicode_frac() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mn>&#xBE;</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_3_1() {
    let expr = "<math><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠸⠌⠼⠓");
}

#[test]
fn fraction_6_4_1() {
    let expr = "<math><mi>y</mi><mo>=</mo><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠽⠀⠐⠶⠀⠷⠭⠨⠌⠼⠃⠾⠰⠄");
}

#[test]
fn fraction_6_4_2() {
    let expr = "<math><mfrac>
        <mrow><mn>2</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠼⠁⠌⠃⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_3() {
    let expr = "<math><mfrac><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow><mn>5</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠸⠌⠼⠉⠨⠌⠼⠑⠾");
}

#[test]
fn fraction_6_4_4() {
    let expr = "<math><mfrac>
    <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
    <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_5() {
    let expr = "<math><mfrac>
        <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_6() {
    let expr = "<math><mtext>speed</mtext><mo>=</mo><mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠎⠏⠑⠑⠙⠀⠐⠶⠀⠷⠙⠊⠎⠞⠁⠝⠉⠑⠨⠌⠞⠊⠍⠑⠾⠰⠄");
}


#[test]
fn msup_7_3_2() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠽");
}

#[test]
fn msup_7_3_3() {
    let expr = "<math><msup><mi>x</mi><mrow><mn>2</mn><mi>y</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠼⠃⠽⠜");
}

#[test]
fn msup_7_3_4() {
    let expr = "<math><msup><mi>x</mi><mi>y</mi></msup><mo>+</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠽⠐⠖⠼⠁");
}

#[test]
fn msup_7_3_6() {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠐⠖⠼⠁⠜⠐⠖⠼⠉");
}

#[test]
fn msup_7_3_7() {
    let expr = "<math><msup><mi>x</mi><mn>⅔</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠌⠉");
}

#[test]
fn msup_7_3_11() {
    let expr = "<math><msup><mi>x</mi><mfrac><mi>a</mi><mi>b</mi></mfrac></msup><mi>y</mi><mo>=</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠔⠷⠁⠨⠌⠃⠾⠽⠀⠐⠶⠀⠭⠰⠄");
}

#[test]
fn msup_7_4_1() {
    let expr = "<math><msup><mi>e</mi><msup><mi>x</mi><mn>2</mn></msup></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠣⠭⠔⠼⠃⠜");
}

#[test]
fn msup_7_4_2() {
    let expr = "<math><msup><mi>e</mi><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠐⠣⠭⠔⠼⠃⠐⠜");
}

#[test]
fn msub_7_4_3() {
    let expr = "<math><msub><mi>P</mi><msub><mi>x</mi><mi>i</mi></msub></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠏⠢⠣⠭⠢⠊⠜");
}

#[test]
fn msup_7_5_1() {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>0045</mn><mo>=</mo>
        <mn>4</mn><mo>.</mo><mn>5</mn><mo>&#xD7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>3</mn></mrow></msup>
        </math>";
    test_braille("UEB", expr, "⠼⠚⠲⠚⠚⠙⠑⠀⠐⠶⠀⠼⠙⠲⠑⠐⠦⠼⠁⠚⠔⠣⠐⠤⠼⠉⠜");
}

#[test]
fn msup_7_5_3() {
    let expr = "<math><msup><mi>a</mi><mrow><mo>-</mo><mn>2</mn><mi>b</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠣⠐⠤⠼⠃⠰⠃⠜");
}

#[test]
fn msup_7_6_2() {
    let expr = "<math><msup><mi mathvariant='normal'>H</mi><mo>+</mo></msup></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖");
}

#[test]
fn msubsup_7_7_1() {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup><mo>=</mo><msubsup><mi>y</mi><mn>2</mn><mn>3</mn></msubsup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠼⠁⠔⠼⠃⠀⠐⠶⠀⠽⠢⠼⠃⠔⠼⠉⠰⠄");
}

#[test]
fn msubsup_7_7_2() {
    let expr = "<math><msub><msup><mi>x</mi><mn>2</mn></msup><mi>k</mi></msub></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠢⠅");
}

#[test]
fn pre_msubsup_7_8_1() {
    // Note: modified because word indicator is not needed
    let expr = "<math><mmultiscripts><mi>U</mi><mprescripts/><mn>92</mn><mn>238</mn></mmultiscripts></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠊⠃⠔⠼⠃⠉⠓⠠⠥");
}

#[test]
fn pre_sup_7_8_2() {
    let expr = "<math><mmultiscripts><mn>2</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>+</mo><mmultiscripts><mn>3</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>=</mo><mmultiscripts><mn>5</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
        </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠔⠐⠤⠼⠃⠐⠖⠔⠐⠤⠼⠉⠀⠐⠶⠀⠔⠐⠤⠼⠑⠰⠄");
}


#[test]
fn sum_7_9_1() {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>x</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover>
            <msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠨⠎⠨⠢⠣⠭⠐⠶⠼⠁⠜⠨⠔⠝⠭⠢⠊⠔⠼⠃");
}

#[test]
fn lim_7_9_2() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mi>a</mi></mrow></munder>
            <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠇⠊⠍⠨⠢⠣⠭⠳⠕⠁⠜⠋⠐⠣⠭⠐⠜⠀⠐⠶⠀⠼⠁⠰⠄");
}

#[test]
fn sqrt_8_1_1() {
    let expr = "<math><msqrt><mn>9</mn></msqrt><mo>=</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠼⠊⠬⠀⠐⠶⠀⠼⠉");
}

#[test]
fn sqrt_8_1_2() {
    let expr = "<math><mi>r</mi><mo>=</mo>
        <msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠗⠀⠐⠶⠀⠩⠭⠔⠼⠃⠐⠖⠽⠔⠼⠃⠬⠰⠄");
}

#[test]
fn sqrt_8_1_3() {
    let expr = "<math>
    <msqrt>
      <mfrac>
        <mrow><mn>783.2</mn><mo>&#xD7;</mo><mn>6.547</mn></mrow>
        <mn>0.4628</mn>
      </mfrac>
    </msqrt>
  </math>";
    test_braille("UEB", expr, "⠰⠰⠩⠷⠼⠛⠓⠉⠲⠃⠐⠦⠼⠋⠲⠑⠙⠛⠨⠌⠼⠚⠲⠙⠋⠃⠓⠾⠬");
}

#[test]
fn sqrt_8_1_4() {
    let expr = "<math>
      <mi>x</mi> <mo>=</mo>
      <mfrac>
       <mrow>
        <mo>&#x2212;</mo><mi>b</mi><mo>&#x00B1;</mo>
        <msqrt>
          <msup><mi>b</mi> <mn>2</mn></msup>
          <mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi>
        </msqrt>
        </mrow>
       <mrow><mn>2</mn><mi>a</mi></mrow>
      </mfrac>
      </math>
    ";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠀⠐⠶⠀⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾⠰⠄");
}

#[test]
fn root_8_2_1() {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠔⠼⠉⠼⠓⠬⠀⠐⠶⠀⠼⠃");
}

#[test]
fn root_8_2_2() {
    let expr = "<math><mi>q</mi><mo>=</mo>
        <mroot><mrow>
        <msup><mi>x</mi><mn>3</mn></msup><mo>+</mo>
        <msup><mi>y</mi><mn>3</mn></msup><mo>+</mo>
        <msup><mi>z</mi><mn>3</mn></msup>
        </mrow><mn>3</mn></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠟⠀⠐⠶⠀⠩⠔⠼⠉⠭⠔⠼⠉⠐⠖⠽⠔⠼⠉⠐⠖⠵⠔⠼⠉⠬⠰⠄");
}

#[test]
fn root_8_2_3() {
    let expr = "<math><mroot><mrow><mi>x</mi><mi>y</mi></mrow><mrow><mi>m</mi><mi>n</mi></mrow></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠣⠍⠝⠜⠭⠽⠬");
}

#[test]
fn root_8_2_4() {
    let expr = "<math>
        <msup><mn>81</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></msup> <mo>=</mo>
        <msup><mrow><mo>(</mo><mroot><mn>81</mn><mn>4</mn></mroot><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo>
        <msup><mrow><mo>(</mo><msqrt><msqrt><mn>81</mn></msqrt></msqrt><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo>
        <msup><mrow><mo>(</mo><msqrt><mn>9</mn></msqrt><mo>)</mo></mrow><mn>3</mn></msup>  <mo>=</mo>
        <msup><mn>3</mn><mn>3</mn></msup>
        <mo>=</mo> <mn>27</mn> </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠼⠓⠁⠔⠼⠉⠌⠙⠀⠐⠶⠀⠐⠣⠩⠔⠼⠙⠼⠓⠁⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠐⠣⠩⠩⠼⠓⠁⠬⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠐⠣⠩⠼⠊⠬⠐⠜⠔⠼⠉⠀⠐⠶⠀⠼⠉⠔⠼⠉⠀⠐⠶⠀⠼⠃⠛⠰⠄");
}

#[test]
fn root_letter_base() {
    // none of the guides cover this case, but it seems that an a-j base needs a grade 1 indicator
    let expr = "<math><mroot><mi>b</mi><mn>3</mn></mroot><mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_braille("UEB", expr, "⠰⠩⠔⠼⠉⠰⠃⠬⠩⠔⠼⠉⠭⠬");
}

#[test]
fn spacing_9_3_1_1() {
    let expr = "<math> <mi>Sin</mi><mo>&#x2061;</mo> <mn>30</mn> </math>";
    test_braille("UEB", expr, "⠠⠎⠔⠼⠉⠚");
}

#[test]
fn spacing_9_3_1_2() {
    let expr = "<math><mn>3</mn><mi>tan</mi><mn>45</mn><mo>&#xB0;</mo></math>";
    test_braille("UEB", expr, "⠼⠉⠞⠁⠝⠼⠙⠑⠘⠚");
}

#[test]
fn spacing_9_3_1_3() {
    let expr = "<math><mn>4</mn><mi>cos</mi><mn>5</mn><mi>x</mi></math>";
    test_braille("UEB", expr, "⠼⠙⠰⠉⠕⠎⠼⠑⠭");
}

#[test]
fn spacing_9_3_2_1() {
    let expr = "<math><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠀⠰⠽");
}

#[test]
fn spacing_9_3_2_2() {
    let expr = "<math><mi>sin</mi><mi>&#x3B8;</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠨⠹");
}

#[test]
fn spacing_9_3_2_3() {
    let expr = "<math><mi>Sec</mi><mi>A</mi></math>";
    test_braille("UEB", expr, "⠠⠎⠑⠉⠠⠁");
}

#[test]
fn spacing_9_3_2_4() {
    let expr = "<math><mi>log</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠐⠣⠭⠐⠖⠽⠐⠜");
}

#[test]
fn spacing_9_3_2_5() {
    let expr = "<math><mi>Lim</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠇⠊⠍⠷⠭⠨⠌⠼⠃⠾");
}

#[test]
fn spacing_9_3_3_1() {
    let expr = "<math><mi>x</mi><mi>sin</mi><mn>60</mn></math>";
    test_braille("UEB", expr, "⠰⠭⠀⠎⠔⠼⠋⠚");
}

#[test]
fn spacing_9_3_3_2() {
    let expr = "<math><mi>x</mi><mrow><mi>Sin</mi><mo>&#x2061;</mo><mn>60</mn></mrow></math>";
    test_braille("UEB", expr, "⠭⠠⠎⠔⠼⠋⠚");
}

#[test]
fn spacing_9_3_3_3() {
    let expr = "<math><mi>X</mi><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠭⠀⠇⠕⠛⠀⠰⠽");
}

#[test]
fn spacing_9_3_3_4() {
    let expr = "<math><mi>x</mi><mi>Log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠠⠇⠕⠛⠀⠰⠽");
}

#[test]
fn spacing_9_3_3_5() {
    let expr = "<math>
        <mi>sin</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>A</mi><mo>+</mo><mi>B</mi><mo>)</mo> </mrow>
        <mo>=</mo><mi>sin</mi><mo>&#x2061;</mo><mi>A</mi><mi>cos</mi><mo>&#x2061;</mo>  <mi>B</mi>
        <mo>+</mo><mi>cos</mi><mo>&#x2061;</mo><mi>A</mi><mi>sin</mi><mo>&#x2061;</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠐⠣⠠⠁⠐⠖⠠⠃⠐⠜⠀⠐⠶⠀⠎⠔⠠⠁⠀⠉⠕⠎⠠⠃⠐⠖⠉⠕⠎⠠⠁⠀⠎⠔⠠⠃");
}

#[test]
fn spacing_9_3_3_6() {
    let expr = "<math><mi>sin</mi><mn>2</mn><mi>&#x3B2;</mi><mo>=</mo>
                <mn>2</mn><mi>sin</mi><mi>&#x3B2;</mi><mi>cos</mi><mi>&#x3B2;</mi></math>";
    test_braille("UEB", expr, "⠎⠔⠼⠃⠨⠃⠀⠐⠶⠀⠼⠃⠎⠊⠝⠨⠃⠉⠕⠎⠨⠃");
}

#[test]
fn text_9_7_1_mtext() {
    let expr = "<math><mtext>Pr</mtext><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mtext>Pr</mtext><mi>A</mi><mo>+</mo><mtext>Pr</mtext><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃");
}

#[test]
fn text_9_7_1() {
    // ugly as the MathML is with non-breaking space in mo's, this is a WIRIS editor output
    let expr = "<math><mi>Pr</mi><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mi>and</mi><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mi>Pr</mi><mi>A</mi><mo>+</mo><mi>Pr</mi><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃");
}

#[test]
fn stat_9_7_2() {
    let expr = "<math><mi>Exp</mi><mo>(</mo><mi>R</mi><mo>)</mo><mo>=</mo>
                            <mfrac><mi>n</mi><mn>2</mn></mfrac><mo>+</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠑⠭⠏⠐⠣⠠⠗⠐⠜⠀⠐⠶⠀⠷⠝⠨⠌⠼⠃⠾⠐⠖⠼⠁⠰⠄");
}

#[test]
fn set_10_1() {
    let expr = "<math><mi>A</mi><mo>=</mo>
        <mfenced open='{' close='}'> 
        <mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mn>4</mn></mrow>
    </mfenced></math>";
    test_braille("UEB", expr, "⠠⠁⠀⠐⠶⠀⠸⠣⠼⠁⠂⠀⠼⠃⠂⠀⠼⠉⠂⠀⠼⠙⠸⠜");
}

#[test]
fn set_10_3() {
    let expr = "<math><mn>3</mn><mo>&#x2208;</mo><mi>A</mi><mo>&#x2229;</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠼⠉⠀⠘⠑⠀⠠⠁⠨⠦⠠⠃");
}

#[test]
fn set_10_4() {
    let expr = "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>&#x2282;</mo><mi>A</mi><mo>&#x222A;</mo><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠁⠨⠦⠠⠃⠀⠘⠣⠀⠠⠁⠨⠖⠠⠃");
}
#[test]
fn set_10_5() {
    let expr = "<math><msup><mi>A</mi><mo>'</mo></msup><mo>∪</mo><msup><mi>B</mi><mo>'</mo></msup><mo>=</mo>
                        <msup><mrow><mo>(</mo><mi>A</mi><mo>∩</mo><mi>B</mi><mo>)</mo></mrow><mo>'</mo></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠠⠁⠶⠨⠖⠠⠃⠶⠀⠐⠶⠀⠐⠣⠠⠁⠨⠦⠠⠃⠐⠜⠶⠰⠄");
}

#[test]
fn set_10_6() {
    // Note: example uses the wrong char "├" in the display -- should be "⊢"
    let expr = "<math><mo>[</mo><mo>(</mo><mi>p</mi><mo>&#x2228;</mo><mi>q</mi><mo>)</mo><mo>&#x2227;</mo><mo>&#xAC;</mo><mi>p</mi><mo>]</mo>
                <mo>⊢</mo><mi>q</mi></math>";
    // Acceptable: GTM does uses a G1 passage indicator: "⠰⠰⠰⠨⠣⠐⠣⠏⠈⠖⠟⠐⠜⠈⠦⠈⠹⠏⠨⠜⠀⠸⠒⠀⠟⠰⠄"
    // However, the BANA G1 standing alone rule ("...before a single letter standing alone") applies, so start in G2 mode.
    // Corrected to remove the passage indicator
    test_braille("UEB", expr, "⠨⠣⠐⠣⠏⠈⠖⠟⠐⠜⠈⠦⠈⠹⠏⠨⠜⠀⠸⠒⠀⠰⠟");
}

#[test]
fn example_11_5_1_2() {
    let expr = "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠙⠽⠨⠌⠙⠭⠾");
}

#[test]
fn example_11_5_1_3() {
    let expr = "<math><mi>f</mi><mo>'</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    // Acceptable: GTM uses a G1 start indicator: "⠰⠰⠋⠶⠐⠣⠭⠐⠜"
    // However, BANA says don't use a word indicator if G1 is in first 3 cells (the ':' needs it)
    // Corrected to avoid word indicator
    test_braille("UEB", expr, "⠋⠰⠶⠐⠣⠭⠐⠜");
}

#[test]
fn example_11_5_1_4() {
    let expr = "<math><mfrac><mrow><mo>&#x2202;</mo><mi>y</mi></mrow><mrow><mo>&#x2202;</mo><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠈⠙⠽⠨⠌⠈⠙⠭⠾");
}

#[test]
fn example_11_5_2() {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>2</mn><mn>3</mn></msubsup><mo>(</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>d</mo><mi>x</mi>
        <mo>=</mo><msubsup><mfenced open='[' close=']'><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>x</mi></mrow></mfenced><mn>2</mn><mn>3</mn></msubsup>
        <mo>=</mo><mo>(</mo><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><mn>3</mn><mo>)</mo><mo>-</mo><mo>(</mo><msup><mn>2</mn><mn>2</mn></msup><mo>+</mo><mn>2</mn><mo>)</mo>
        <mo>=</mo><mn>12</mn><mo>-</mo><mn>6</mn><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠮⠢⠼⠃⠔⠼⠉⠐⠣⠼⠃⠭⠐⠖⠼⠁⠐⠜⠙⠭⠀⠐⠶⠀⠨⠣⠭⠔⠼⠃⠐⠖⠭⠨⠜⠢⠼⠃⠔⠼⠉⠀⠐⠶⠀⠐⠣⠼⠉⠔⠼⠃⠐⠖⠼⠉⠐⠜⠐⠤⠐⠣⠼⠃⠔⠼⠃⠐⠖⠼⠃⠐⠜⠀⠐⠶⠀⠼⠁⠃⠐⠤⠼⠋⠀⠐⠶⠀⠼⠋⠰⠄");
}

#[test]
fn example_11_5_3() {
    // modified to use "shape" as recommended in a comment on this example
    let expr = "<math><msub><mmultiscripts><mi>C</mi><mprescripts/><none/><mi>n</mi></mmultiscripts><mi>r</mi></msub><mo>=</mo>
            <mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac><mo>)</mo><mo>=</mo>
            <mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mi>r</mi><mo>!</mo><mo>(</mo><mi>n</mi><mo>-</mo><mi>r</mi><mo>)</mo><mo>!</mo></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠔⠝⠠⠉⠢⠗⠀⠐⠶⠀⠐⠣⠝⠰⠻⠗⠐⠜⠀⠐⠶⠀⠷⠝⠖⠨⠌⠗⠖⠐⠣⠝⠐⠤⠗⠐⠜⠖⠾⠰⠄");
}

#[test]
fn example_11_5_4() {
    let expr = "<math><mi>a</mi><mo>&#x2217;</mo><mo>(</mo><mi>b</mi><mo>&#x25E6;</mo><mi>c</mi><mo>)</mo>
        <mo>=</mo><mo>(</mo><mi>a</mi><mo>&#x2217;</mo><mi>b</mi><mo>)</mo><mo>&#x25E6;</mo><mo>(</mo><mi>a</mi><mo>&#x2217;</mo><mi>c</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠁⠐⠔⠐⠣⠃⠐⠴⠉⠐⠜⠀⠐⠶⠀⠐⠣⠁⠐⠔⠃⠐⠜⠐⠴⠐⠣⠁⠐⠔⠉⠐⠜");
}

#[test]
fn example_11_5_5_2() {
    let expr = "<math>
    <msup>
      <mi>f</mi>
      <mrow> <mo>&#x2212;</mo> <mn>1</mn> </mrow>
    </msup>
    <mo>:</mo>
    <mi>Y</mi>
    <mo>&#x2192;</mo>
    <mi>X</mi>
  </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠋⠔⠣⠐⠤⠼⠁⠜⠒⠀⠠⠽⠀⠳⠕⠀⠠⠭⠰⠄");
}

#[test]
fn example_11_5_5_3() {
    // this comes from MathJax
    let expr = "<math>
        <mi mathvariant='normal'>&#x2200;</mi>
        <mi>y</mi>
        <mo>&#x2208;</mo>
        <mi>Y</mi>
        <mstyle scriptlevel='0'>  <mspace width='0.278em'></mspace> </mstyle>
        <mi mathvariant='normal'>&#x2203;</mi>
        <mi>x</mi>
        <mo>&#x2208;</mo>
        <mi>X</mi>
  </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠘⠁⠽⠀⠘⠑⠀⠠⠽⠀⠘⠢⠭⠀⠘⠑⠀⠠⠭⠰⠄");
}

#[test]
fn example_11_5_6() {
    let expr = "<math> <mo>{</mo>
            <mo>(</mo> <mi>x</mi> <mo>,</mo> <mi>y</mi> <mo>)</mo>
            <mo>|</mo>
            <mi>x</mi> <mo>+</mo> <mi>y</mi> <mo>=</mo> <mn>6</mn>
        <mo>}</mo> </math>";
    // Acceptable: GTM uses a G1 passage indicator: "⠰⠰⠰⠸⠣⠐⠣⠭⠂⠀⠽⠐⠜⠀⠸⠳⠀⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋⠸⠜⠰⠄"
    // However, the BANA G1 standing alone rule ("...before a single letter standing alone") applies, so start in G2 mode.
    // Corrected to remove the passage indicator
    test_braille("UEB", expr, "⠸⠣⠐⠣⠭⠂⠀⠰⠽⠐⠜⠀⠸⠳⠀⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋⠸⠜");
}

#[test]
fn example_11_6_math_variant() {
    let expr = "<math><mi mathvariant='fraktur'>R</mi></math>";
    test_braille("UEB", expr, "⠈⠆⠰⠠⠗");
}

#[test]
fn example_11_6() {
    let expr = "<math><mi>ℜ</mi></math>";
    test_braille("UEB", expr, "⠈⠆⠰⠠⠗");
}

#[test]
fn bar_over_12_1_1() {
    let expr = "<math><mover><mi>x</mi><mo>_</mo></mover><mo>=</mo>
        <mfrac><mrow><mn>10</mn><mo>+</mo><mn>11</mn><mo>+</mo><mn>12</mn></mrow><mn>3</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠱⠀⠐⠶⠀⠷⠼⠁⠚⠐⠖⠼⠁⠁⠐⠖⠼⠁⠃⠨⠌⠼⠉⠾⠰⠄");
}

#[test]
fn bar_under_12_1_2() {
    let expr = "<math><munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>_</mo></munder></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱");
}

#[test]
fn bar_menclose_12_1_2() {
    let expr = "<math><menclose notation='bottom'><mi>x</mi><mo>+</mo><mi>y</mi></menclose></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱");
}

#[test]
fn dot_12_1_4() {
    let expr = "<math><mn>0</mn><mo>.</mo><mover><mn>3</mn><mo>.</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠣⠼⠉⠜⠘⠲");
}

#[test]
fn dot_12_1_5() {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>56</mn><mover><mn>1</mn><mo>&#x2D9;</mo></mover>
            <mn>2</mn><mover><mn>3</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠑⠋⠣⠼⠁⠜⠘⠲⠼⠃⠣⠼⠉⠜⠘⠲");
}

#[test]
fn dot_12_1_6_single() {
    let expr = "<math><mover><mi>x</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠘⠲");
}

#[test]
fn dot_12_1_6_double() {
    let expr = "<math><mover><mi>x</mi><mo>&#xA8;</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠨⠔⠣⠲⠲⠜");
}

#[test]
fn hat_12_1_7() {
    let expr = "<math><mi>A</mi><mover><mi>B</mi><mo>^</mo></mover><mi>C</mi></math>";
    // Acceptable: GTM uses a G1 start indicator: "⠰⠰⠠⠁⠠⠃⠐⠱⠠⠉"
    // BANA says use a word indicator if G1 not in first 3 cells (modified it to not count cap indicators since that helps with GTM compatibility)
    // Corrected to skip the G1 indicator at the start (it's debatable as to which is better)
    test_braille("UEB", expr, "⠠⠁⠠⠃⠰⠐⠱⠠⠉");
}

#[test]
fn arrow_over_12() {
    // This comes from https://uebonline.org/wp-content/uploads/2021/05/Unified-English-Braille-Extension-Maths-Training-Manual-First-Edition-Rev-4.pdf
    let expr = "<math><mover><mi>x</mi><mo>→</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠰⠘⠱");
}

#[test]
fn arrow_under_12() {
    // This comes from https://uebonline.org/wp-content/uploads/2021/05/Unified-English-Braille-Extension-Maths-Training-Manual-First-Edition-Rev-4.pdf
    let expr = "<math><munder><mi>x</mi><mo>→</mo></munder></math>";
    test_braille("UEB", expr, "⠭⠰⠠⠘⠱");
}

#[test]
fn bar_12_2_1() {
    let expr = "<math><msup><mi>x</mi><mover><mi>y</mi><mo>&#xAF;</mo></mover></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠱⠜");
}

#[test]
fn bar_12_2_2() {
    let expr = "<math><mover><msup><mi>x</mi><mi>y</mi></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠔⠽⠜⠱");
}

#[test]
fn shape_14_1_1_1() {
    let expr = "<math><mo>&#x25B3;</mo><mo>&#xA0;</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠀⠠⠠⠁⠃⠉");
}

#[test]
fn shape_14_1_2_1() {
    let expr = "<math><mo>&#x25B3;</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠱⠠⠠⠁⠃⠉");
}

#[test]
fn shape_14_1_2_2() {
    // the <mo> for the shapes are wrong -- but it isn't clear what they should be (from WIRIS editor)
    let expr = "<math><mo>{</mo><mo>&#x25A1;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25CD;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25B2;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25A7;</mo><mo>&#xA0;</mo><mo>&#x2026;</mo><mo>}</mo></math>";
    test_braille("UEB", expr, "⠸⠣⠰⠫⠼⠙⠱⠂⠀⠨⠫⠿⠱⠂⠀⠸⠫⠼⠉⠱⠂⠀⠨⠫⠼⠙⠀⠲⠲⠲⠸⠜");
}

#[test]
fn binomial_14_3_3_2() {
    let expr = "<math><mfenced><mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac></mfenced></math>";
    test_braille("UEB", expr, "⠐⠣⠝⠰⠻⠗⠐⠜");
}

#[test]
fn binomial_14_3_3_2_mtable() {
    let expr = "<math><mrow intent='binomial($n,$r)'>
            <mo>(</mo>
                <mtable>
                <mtr><mtd><mi arg='n'>n</mi></mtd></mtr>
                <mtr><mtd><mi arg='r'>r</mi></mtd></mtr>
                </mtable>
            <mo>)</mo>
        </mrow></math>";
    test_braille("UEB", expr, "⠐⠣⠝⠰⠻⠗⠐⠜");
}

#[test]
fn chem_16_2_8() {
    let expr = "<math><mi>Ca</mi><msub><mrow><mo>(</mo><mi>OH</mi><mo>)</mo></mrow><mn>2</mn></msub></math>";
    // Acceptable: GTM does not use a G1 start indicator: "⠠⠉⠁⠐⠣⠠⠕⠠⠓⠐⠜⠰⠢⠼⠃"
    // However, BANA says use a word indicator if G1 not in first 3 cells (it is before the subscript near the end); use passage if >=2 whitespace
    // This seems like a debateable choice in this case since there is only one G1 indicator, but that's the BANA guidance so...
    // Corrected to use word indicator
    test_braille("UEB", expr, "⠰⠰⠠⠉⠁⠐⠣⠠⠕⠠⠓⠐⠜⠢⠼⠃");
}

#[test]
fn chem_16_2_9() {
    // from mhchem -- \ce{CuSO4·5H2O}
    let expr = "<math>
        <mrow>
        <mrow>
            <mi data-mjx-auto-op='false'>CuSO</mi>
        </mrow>
        <msub>
            <mrow>
            <mrow>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
            </mrow>
            </mrow>
            <mrow>
            <mrow>
                <mpadded height='0'>
                <mn>4</mn>
                </mpadded>
            </mrow>
            </mrow>
        </msub>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mrow>
            <mo>&#x22C5;</mo>
        </mrow>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mn>5</mn>
        <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
        </mstyle>
        <mrow>
            <mi mathvariant='normal'>H</mi>
        </mrow>
        <msub>
            <mrow>
            <mrow>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
            </mrow>
            </mrow>
            <mrow>
            <mrow>
                <mpadded height='0'>
                <mn>2</mn>
                </mpadded>
            </mrow>
            </mrow>
        </msub>
        <mrow>
            <mi mathvariant='normal'>O</mi>
        </mrow>
        </mrow>
    </math>";
    // Acceptable: GTM does not use a G1 start indicator: "⠠⠉⠥⠠⠎⠠⠕⠰⠢⠼⠙⠐⠲⠼⠑⠠⠓⠢⠼⠃⠠⠕"
    // However, BANA says use a word indicator if G1 not in first 3 cells (it is before the subscript); use passage if >=2 whitespace
    // This seems like a debatable choice in this case since there is only one G1 indicator, but that's the BANA guidance so...
    // Corrected to use word indicator
    test_braille("UEB", expr, "⠰⠰⠠⠉⠥⠠⠎⠠⠕⠢⠼⠙⠐⠲⠼⠑⠠⠓⠢⠼⠃⠠⠕");
}

#[test]
fn chem_16_2_10() {
    let expr = "<math><mmultiscripts><mi mathvariant='normal'>H</mi><none/><mo>+</mo></mmultiscripts></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖");
}

#[test]
fn chem_16_2_11() {
    let expr = "<math>
        <mi mathvariant='normal'>S</mi>
        <mmultiscripts> <mi mathvariant='normal'>O</mi> <mn>4</mn> <mrow><mo>-</mo><mo>-</mo></mrow>  </mmultiscripts>
    </math>";
    test_braille("UEB", expr, "⠠⠎⠠⠕⠰⠢⠼⠙⠔⠣⠐⠤⠐⠤⠜");
}

#[test]
fn chem_16_2_13() {
    let expr = "<math>
        <mmultiscripts><mi>Fe</mi><none/><mi>III</mi></mmultiscripts>
        <mmultiscripts><mi>Cl</mi><mn>3</mn><none/></mmultiscripts>
    </math>";
    test_braille("UEB", expr, "⠰⠰⠠⠋⠑⠔⠣⠠⠠⠊⠊⠊⠜⠠⠉⠇⠢⠼⠉");
}


// Extra tests targeted at special cases in MathCAT
#[test]
fn number_space_before() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn></math>";
    test_braille("UEB", expr, "⠀⠼⠃");
}

#[test]
fn number_space_after() {
    let expr = "<math><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("UEB", expr, "⠼⠃⠀");
}

#[test]
fn number_space_before_and_after() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("UEB", expr, "⠀⠼⠃⠀");
}

// extra tests targeted at contractions based on function names
#[test]
fn contractions_1() {
    let expr = "<math>
        <mi>sech</mi><mo>&#x2061;</mo><mi>x</mi><mo>+</mo>
        <mi>cosh</mi><mo>&#x2061;</mo><mi>y</mi><mo>+</mo>
        <mi>arccos</mi><mo>&#x2061;</mo><mi>t</mi>
    </math>";
    // Note: "arccos" does not use the "cc" contraction -- RUEB 10.6.5 lists "arccosine" without the contraction
    test_braille("UEB", expr, "⠎⠑⠡⠀⠭⠐⠖⠉⠕⠩⠀⠽⠐⠖⠜⠉⠉⠕⠎⠀⠰⠞");
}
#[test]
fn contractions_2() {
    let expr = "<math><mi>ker</mi><mo>&#x2061;</mo><mi>h</mi></math>";
    test_braille("UEB", expr, "⠅⠻⠀⠰⠓");
}

#[test]
fn contractions_3() {
    let expr = "<math><mi>argument</mi><mo>&#x2061;</mo><mo>(</mo><mi>f</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠜⠛⠥⠰⠞⠐⠣⠋⠐⠜");
}

#[test]
fn contractions_4() {
    let expr = "<math><mtext>error&#xA0;function&#xA0;</mtext><mi>erf</mi></math>";
    test_braille("UEB", expr, "⠻⠗⠕⠗⠀⠋⠥⠝⠉⠰⠝⠀⠻⠋");
}

#[test]
fn contractions_5() {
    let expr = "<math><mi>Real</mi><mo>(</mo><mi>z</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠗⠂⠇⠐⠣⠵⠐⠜");
}
