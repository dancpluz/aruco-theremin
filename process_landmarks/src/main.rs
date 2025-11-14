use mediapipe_rs::tasks::vision::HandLandmarkerBuilder;

fn parse_args() -> Result<(String, String, Option<String>), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        return Err(format!(
            "Usage: {} <model_path> <image_path> [output_image_path]",
            args[0]
        )
        .into());
    }
    Ok((args[1].clone(), args[2].clone(), args.get(3).cloned()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (model_path, img_path, output_path) = parse_args()?;

    let mut input_img = image::open(&img_path)?;
    let hand_landmark_results = HandLandmarkerBuilder::new()
        .min_hand_detection_confidence(0.5)
        .build_from_file(model_path)?
        .detect(&input_img)?;

    println!("{}", hand_landmark_results);

    // Draw landmarks without specifying custom connections
    if let Some(output_path) = output_path {
        for result in hand_landmark_results.iter() {
            // Use the default drawing method, if it exists
            result.draw(&mut input_img); 
        }
        input_img.save(&output_path)?;
    }

    Ok(())
}