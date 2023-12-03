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
        gap: 5px;
    }}
    .subtractions {{
        grid-column: 2;
        gap: 5px;
    }}
    .equation {{
        padding: 2 2;
        margin: 5 10;
    }}
    .operand-one {{
    }}
    .operand-two {{
    }}
    .operator {{
    }}
    .answer {{
        border: thin solid black;
    }}
    .strategies {{
        border: thin solid black;
        padding: 5 10;
        margin: 2;
    }}
    .strategy {{
        padding: 2 10;
        display:inline-block;
    }}
    </style>
    </head>
    <body>"#
    );

    let strategies = r#"<span class="strategies">
    <span class="strategy">ta</span>
    <span class="strategy">co</span>
    <span class="strategy">d</span>
    <span class="strategy">nd</span>
    <span class="strategy">rf</span>
    <span class="strategy">fn</span>
    <span class="strategy">btt</span>
    </span>"#;
    let addition_list = create_addition_list(20, 3, 18);
    println!(r#"<div class="additions">"#);
    for addition_equation in addition_list {
        println!(
            r#"<div class="equation"><span class="operand-one">{}</span><span class="operator">{}</span><span class="operand-two">{}</span>
            {}
            <span class="equal-sign">=</span>
            <span class="answer">&emsp;&emsp;</span></div>"#,
            addition_equation.first_operand,
            addition_equation.operator,
            addition_equation.second_operand,
            strategies,
        );
    }
    println!(r#"</div>"#);
    println!(r#"</body></html>"#);
}
