use structopt::StructOpt;
use std::vec::Vec;
use plotters::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "collatz", about = "Fun with the 3x+1 problem.")]
struct Cli {
    lower: Option<i128>,
    upper: Option<i128>,
}

#[derive(Debug)]
struct CollatzResult {
    iterations: i128,
    max:        i128,
    input:      i128,
}
impl CollatzResult {
    fn set_max(&mut self, new_max:i128) {
        self.max = new_max;
    }
    fn inc_iterations(&mut self) {
        self.iterations += 1;
    }
}

fn main() {
    let opt = Cli::from_args();
    let mut lower:i128 = 0;
    let mut upper:i128 = 27;
    let mut iterations_collection = Vec::new();

    if let Some(l) = &opt.lower {
        lower = *l;
    }

    if let Some(u) = &opt.upper {
        upper = *u;
    }

    for i in lower..upper {
        let mut result = CollatzResult {
            iterations:   0,
            max:       0,
            input:     i,
        };
        three_ecks_plus_one(i, &mut result);
        iterations_collection.push(result.iterations);
        println!("{:?}", result);
    }
    create_histogram(&iterations_collection).unwrap();
}

fn three_ecks_plus_one(num: i128, result: &mut CollatzResult) -> i128 {
    let product:i128;
    if num > result.max {
        result.set_max(num);
    }
    if num < 5 { 
        product = num;
    } else if (num % 2) == 1 { 
        product = three_ecks_plus_one((3 * num) + 1, result);
    } else {
        product = three_ecks_plus_one(num / 2, result);
    }
    result.inc_iterations();
    return product;
}

fn create_histogram(data: &Vec<i128>) -> Result<(), Box<dyn std::error::Error>> {
    const OUT_FILE_NAME: &'static str = "histogram.png";
    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 1200)).into_drawing_area();
    let biggest_x = data.iter().max().unwrap();
    let biggest_y = 75i128;
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Collatz histogram", ("sans-serif", 30.0))
        .build_cartesian_2d(0i128..*biggest_x, 0i128..biggest_y)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count of integers")
        .x_desc("Iterations")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .margin(3)
            .data(data.iter().map(|x: &i128| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    return Ok(())
}