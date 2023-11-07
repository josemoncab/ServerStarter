/*let a = Path::new("C:\\Program Files\\Eclipse Adoptium\\jdk-17.0.8.7-hotspot\\bin\\java\
    .exe");

let stdout = Command::new("cmd")
.arg("/C")
.arg(a)
.arg("-jar")
.arg("paper.jar")
.stdout(Stdio::piped())
.spawn().unwrap()
.stdout
.ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

let reader = BufReader::new(stdout);

reader
.lines()
.map_while(Result::ok)
.for_each(|line| println!("{}", line));*/