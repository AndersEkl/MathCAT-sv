/// Tests for:
/// *  calculus-related inference
use crate::common::*;


#[test]
fn simple_gradient() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_bold_gradient() {
  let mathml = r#"<math>
        <mi mathvariant="bold">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
       </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_div() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>⋅</mo>
        <mi mathvariant="bold">f</mi>
     </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <divergence data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
    </divergence> 
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_curl() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>&#xD7;</mo>
        <mi mathvariant="bold">f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn gradient_vector() {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn curl_vector() {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mo>&#xD7;</mo>
        <mover>
            <mi>f</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added'>
            <modified-variable data-from-mathml='mover'>
                <mi data-from-mathml='mi'>f</mi>
                <mo data-from-mathml='mo' stretchy='false'>→</mo>
            </modified-variable>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}
