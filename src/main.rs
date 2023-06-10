use clap::{App, Arg};
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = App::new("Kubernetes Manifest Generator")
        .version("1.1")
        .author("Stephen Pawulski")
        .about("Generates Kubernetes manifests")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Sets the name of the application")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets the port of the container")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("IMAGE")
                .help("Sets the container image")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let name = matches.value_of("name").unwrap();
    let image = matches.value_of("image").unwrap();
    let port = matches.value_of("port").unwrap();

    let deployment_manifest = format!(
        r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {}
spec:
  replicas: 1
  selector:
    matchLabels:
      app: {}
  template:
    metadata:
      labels:
        app: {}
    spec:
      containers:
      - name: {}
        image: {}
        ports:
        - containerPort: {}
        resources:
          limits:
            memory: 200Mi
          requests:
            cpu: 100m
            memory: 200Mi"#,
        name, name, name, name, image, port
    );

    let service_manifest = format!(
        r#"apiVersion: v1
kind: Service
metadata:
  name: {}
spec:
  selector:
    app: {}
  ports:
    - protocol: TCP
      port: 80
      targetPort: {}
  type: LoadBalancer"#,
        name, name, port
    );

    let output_dir = format!("{}-manifests", name);
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    let deployment_path = format!("{}/deployment.yaml", output_dir);
    let service_path = format!("{}/service.yaml", output_dir);

    let mut deployment_file =
        File::create(&deployment_path).expect("Failed to create deployment file");
    deployment_file
        .write_all(deployment_manifest.as_bytes())
        .expect("Failed to write to deployment file");

    let mut service_file = File::create(&service_path).expect("Failed to create service file");
    service_file
        .write_all(service_manifest.as_bytes())
        .expect("Failed to write to service file");

    println!("Kubernetes manifests generated successfully!");
}

