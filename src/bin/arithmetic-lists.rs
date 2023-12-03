use math_sheets::*;

fn main() {
    println!(
        r#"<html>
    <head><title>Math Sheet</title>
    <style>
    body {{
        align: center;
    }}
    .equations {{
        display: grid;
        grid-template-columns: auto auto;
        gap: 10px;
    }}
    .additions {{
        grid-column: 1;
        display: grid;
        gap: 5px;
    }}
    .subtractions {{
        grid-column: 2;
        display: grid;
        gap: 5px;
    }}
    .equation {{
        display: flex;
    }}
    .operand-one {{
        margin: 1px;
    }}
    .operand-two {{
        margin: 1px;
    }}
    .operator {{
        margin: 1px;
    }}
    .answer {{
        border: thin solid black;
    }}
    </style>
    </head>
    <body>
    <div class="equations">"#
    );
    let addition_list = create_addition_list(20, 3, 18);
    println!(r#"<div class="additions">"#);
    for addition_equation in addition_list {
        println!(
            r#"<div class="equation"><span class="operand-one">{}</span><span class="operator">{}</span><span class="operand-two">{}</span><span class="equal-sign">=</span><span class="answer">&emsp;&emsp;</span></div>"#,
            addition_equation.first_operand,
            addition_equation.operator,
            addition_equation.second_operand,
        );
    }
    println!(r#"</div>"#);

    let subtraction_list = create_subtraction_list(20, 3, 18);
    println!(r#"<div class="subtractions">"#);
    for subtraction_equation in subtraction_list {
        println!(
            r#"<div class="equation"><span class="operand-one">{}</span><span class="operator">{}</span><span class="operand-two">{}</span><span class="equal-sign">=</span><span class="answer">&emsp;&emsp;</span></div>"#,
            subtraction_equation.first_operand,
            subtraction_equation.operator,
            subtraction_equation.second_operand,
        )
    }
    println!(r#"</div>"#);

    println!("</div></body></html>")
}
