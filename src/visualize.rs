use plotters::prelude::*;
use std::fs::File;
use std::error::Error;
use plotters::coord::types::RangedCoordusize;
use plotters::coord::types::RangedCoordf64;
use csv;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "out7.csv";
    let save_path = "outputs/energy7.png";

    let file = File::open(file_path);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_path(file_path)?;
    let mut ke: Vec<f64> = Vec::new();
    let mut pe: Vec<f64> = Vec::new();

    for res in rdr.records() {
        let record = res?;

        let mut kei: f64 = record[0].parse().unwrap();
        // let kei = kei / 1041192618002647200000000000.0;

        ke.push(kei);
        pe.push(record[1].parse().unwrap());
        // println!("{:?}, {:?}", &record[0], &record[1]);
    }

    let (ke_min, ke_max) = ke.iter().fold((0.0/0.0, 0.0/0.0),|(m,n), v| (v.min(m), v.max(n)));

    let mut backend = BitMapBackend::new(save_path, (600, 400));
    let root:DrawingArea<_,_> = backend.into();
    root.fill(&RGBColor(255,255,255)).unwrap();
    
    let font = ("san-serif", 20);

    let mut chart: ChartContext<BitMapBackend, Cartesian2d<RangedCoordusize, RangedCoordf64>> = ChartBuilder::on(&root)
        .caption("kinetic energy(ke), potential(pe) and total", font.into_font())
        .build_cartesian_2d(0..ke.len(), 0.0..ke_max)?;
        
        // .set_label_area_size(LabelAreaPosition::Top, 40.0)
        // .caption("House Sales in King County", ("sans-serif", 40.0))
        // .build_cartesian_2d(0.0..6000.0, 0.0..10000.0)
        // .unwrap();
    chart.configure_mesh().draw()?;

    let ke_cloned = ke.clone();
    let series = LineSeries::new(ke_cloned.into_iter().enumerate(), &RGBColor(255, 0, 0));
    chart.draw_series(series)?
        .label("ke")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(255, 0, 0)));
    let pe_cloned = pe.clone();
    let series = LineSeries::new(pe_cloned.into_iter().enumerate(), &RGBColor(0, 0, 255));
    chart.draw_series(series)?
        .label("pe")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));

    let mut sum = ke.clone();
    for (kei, pei) in sum.iter_mut().zip(pe.as_slice()) {
        *kei += *pei;
    };
    let series = LineSeries::new(sum.into_iter().enumerate(), &RGBColor(0, 0, 0));
    chart.draw_series(series)?
        .label("total")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 0)));

    chart.configure_series_labels()
        .background_style(&RGBColor(255, 255, 255).mix(0.8))
        .border_style(&RGBColor(10, 10, 10))
        .draw()?;
    
    Ok(())
}


//     let root_area = BitMapBackend::new("outputs/2.5.png", (600, 400))
//                                                         .into_drawing_area();
//     root_area.fill(&WHITE).unwrap();
//     let mut ctx = ChartBuilder::on(&root_area)
//         .set_label_area_size(LabelAreaPosition::Left, 40)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40)
//         .caption("Line Plot Demo", ("sans-serif", 40))
//         .build_cartesian_2d(-10..10, 0..100)
//         .unwrap();

//     ctx.configure_mesh().draw().unwrap();

//     ctx.draw_series(
//         LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
//     ).unwrap();

// }