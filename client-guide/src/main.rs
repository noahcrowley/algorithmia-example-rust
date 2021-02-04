use algorithmia::Algorithmia;
use algorithmia::data::*;
use serde_json::json;

fn main() {
    let api_key = "simJsfTIRQXG0pwayRklahVH2sh1";
    let client = Algorithmia::client(api_key).unwrap();

    let algo = client.algo("demo/Hello");
    let response = algo.pipe("HAL 9000").unwrap();
    println!("{}", response.as_string().unwrap());

    let algo_json = client.algo("nlp/LDA/1.0.0");
    let input = json!({
        "docsList": [
            "The apples are ready for picking",
            "It's apple picking season"
        ]
    });
    let response_json = algo_json.pipe(input);
    let output = response_json.unwrap();
    println!("{}", output.to_json().unwrap());

    let mut algo_options = client.algo("demo/Hello");
    let algo_options = algo_options.timeout(10).stdout(false);
    let response_options = algo_options.pipe("HAL 9001").unwrap();
    println!("{}", response_options.as_string().unwrap());

    // match algo.pipe("HAL 9002") {
    //     Ok(response) => println!("{}", response.as_string().unwrap()),
    //     Err(err) => println!("error calling demo/Hello: {}", err)
    // }

    let algo_error = client.algo("util/WhoopsWrongAlgo");
    match algo_error.pipe("Hello, world!") {
        Ok(_response) => { /* success */ },
        Err(err) => println!("error calling algorithm: {}", err),
    }
    
    // match client.dir("data://noahcrowley/img_directory").create(DataAcl::default()) {
    //     Ok(_response) => println!("directory created"),
    //     Err(err) => println!("error creating directory: {}", err)
    // }

    let img_directory = client.dir("data://noahcrowley/img_directory");

    // from Anthony Nowell: nit: testing for == false followed by an else clause is less natural to read
    // 
    // if img_directory.exists().unwrap() == false {
    //     img_directory.create(DataAcl::from(ReadAcl::Public)).unwrap();
    // } else {
    //     println!("img_directory exists");
    // }

    if img_directory.exists().unwrap() {
        println!("img_directory exists");
     } else { 
         img_directory.create(DataAcl::from(ReadAcl::Public)).unwrap();
     }


    // let img_file = "data://.my/img_directory/friends.jpg";
    // if client.file(img_file).exists().unwrap() == false {
    //     client.file(img_file).put("data/friends.jpg").unwrap();
    // }

    let img_file = "data://.my/img_directory/friends.jpg";

    // if client.file(img_file).exists().unwrap() == false {
    //     img_directory.put_file("../data/friends.jpg").unwrap();
    // } else {
    //     println!{"file exists"};
    // }

    if client.file(img_file).exists().unwrap() {
        println!("file exists");
    } else {
        img_directory.put_file("../data/friends.jpg").unwrap();
    }

    let algo_cv = client.algo("dlib/FaceDetection/0.2.1");
    let input_cv = json!({
        "images": [
            {
                "url": "data://.my/img_directory/friends.jpg",
                "output": "data://.algo/temp/detected_faces.png"
            }
        ]
    });

    // let response_cv = algo_cv.pipe(input_cv);
    // let output_cv = response_cv.unwrap();
    // println!("{}", output_cv.to_json().unwrap());

    match algo_cv.pipe(input_cv) {
        Ok(response) => println!("{}", response.to_json().unwrap()),
        Err(err) => println!("error calling FaceDetection algo: {}", err)
    };

    let download_uri = "data://.algo/dlib/FaceDetection/temp/detected_faces.png";
    if client.file(download_uri).exists().unwrap() {
        let mut png_reader = client.file(download_uri).get().unwrap();
        let mut png = std::fs::File::create("../data/detected_faces.png").unwrap();
        std::io::copy(&mut png_reader, &mut png).unwrap();
    };
}
