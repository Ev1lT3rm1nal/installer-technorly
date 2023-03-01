use question::{Answer, Question};
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use which::{which, which_all};

fn main() {
    // Comprobar si Chocolatey está instalado

    println!("Comprobando si Chocolatey está instalado...");
    let output = which("choco");

    if output.is_err() {
        // Chocolatey no está instalado, instalarlo
        println!("Chocolatey no está instalado, instalando...");
        let install_command = "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))";

        let mut process = Command::new("powershell")
            .stderr(Stdio::piped())
            .arg(install_command)
            .spawn()
            .expect("Error al ejecutar el comando de instalación de Chocolatey");

        process.wait().expect("Error al correr el comando");

        // Añadir Chocolatey al PATH
        env::set_var(
            "PATH",
            format!(
                "{};C:\\ProgramData\\chocolatey\\bin",
                env::var("PATH").unwrap()
            ),
        );
    }

    // Comprobar si OpenJDK 17 está instalado
    println!("\n\nComprobando si OpenJDK 17 está instalado...");

    // Obtiene todos los ejecutables en el PATH de java y ve si existe JAVA_HOME
    let executables = which_all("java");

    // comprueba si alguno de los ejecutables es openjdk 17
    let mut executable_path: Option<PathBuf> = None;

    if let Ok(executables) = executables {
        for executable in executables {
            let output = Command::new(executable.clone())
                .arg("-version")
                .output()
                .expect("Error al ejecutar el comando 'java -version'");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stderr).to_string();
                // Comprueba si el stdout contiene "openjdk version \"17" y OpenJDK Runtime Environment (build 17
                if stdout.contains("openjdk version \"17")
                    && stdout.contains("OpenJDK Runtime Environment (build 17")
                {
                    executable_path = Some(executable);
                    break;
                }
            }
        }
    }

    /*
       En caso de que si haya, comproba si JAVA_HOME está configurado y es el mismo que el directorio de OpenJDK 17
       En caso de que no, configúralo
       En caso de que no, instala OpenJDK 17
    */

    if let Some(executable) = executable_path {
        // get java.home it through java -XshowSettings:properties -version
        let output = Command::new(executable)
            .arg("-XshowSettings:properties")
            .arg("-version")
            .output()
            .expect("Error al obtener la propiedad 'java.home'");

        let stdout_s = String::from_utf8_lossy(&output.stderr).to_string();

        let java_home = stdout_s.split("java.home = ").collect::<Vec<&str>>()[1]
            .split("\n")
            .collect::<Vec<&str>>()[0]
            .to_string();

        // get env var JAVA_HOME
        let java_home_env = env::var("JAVA_HOME").unwrap_or("".to_string());

        if java_home_env.is_empty() || java_home_env != java_home {
            // Configurar JAVA_HOME
            println!("Configurando JAVA_HOME...");

            // setx Java_Home "C:\Program Files\Java\jdk-17.0.1"
            Command::new("powershell")
                .arg(format!("setx JAVA_HOME \"{}\"", java_home))
                .output()
                .expect("Error al ejecutar el comando 'setx JAVA_HOME'");
        }
    } else {
        install("openjdk17 --version=17.0.2", "OpenJDK 17");
    }

    // Comprobar si Maven está instalado
    check_and_install("mvn", "maven --version=3.8.4", "Maven");

    // Comprobar si Postgres está instalado
    println!("\n\nComprobando si Postgres está instalado...");

    let output = which("psql");

    if output.is_err() {
        // use gui installer from https://sbp.enterprisedb.com/getfile.jsp?fileid=1258323 with reqwest
        // Postgres no está instalado, instalarlo
        println!("\nPostgres no está instalado, instalando...");
        println!("Descargando instalador de Postgres...");
        let target = "https://sbp.enterprisedb.com/getfile.jsp?fileid=1258323";
        let mut resp = reqwest::blocking::get(target).unwrap();
        let mut file = std::fs::File::create("postgres.exe").unwrap();
        std::io::copy(&mut resp, &mut file).unwrap();

        file.flush().unwrap();
        file.sync_all().unwrap();
        // para de usar el archivo
        drop(file);

        Command::new("postgres.exe")
            .output()
            .expect("Error al ejecutar el comando de instalación de Postgres");
    }

    // Comprobar si Git está instalado
    check_and_install("git", "git", "Git");

    // Comprobar si NodeJS está instalado
    check_and_install("node", "nodejs-lts", "NodeJS");

    // Comprobar si Visual Studio Code está instalado
    let output = which("code");

    if output.is_err() {
        // Preguntar si desea instalar Visual Studio Code
        let ans = Question::new("\n\n¿Desea instalar Visual Studio Code?")
            .acceptable(vec!["s", "n"])
            .default(Answer::YES)
            .show_defaults()
            .until_acceptable()
            .ask();

        if let Some(Answer::YES) = ans {
            // Instalar Visual Studio Code
            install("vscode", "Visual Studio Code");
        }
    }

    // agregar vscode al path
    env::set_var(
        "PATH",
        format!(
            "{};C:\\Program Files\\Microsoft VS Code\\bin",
            env::var("PATH").unwrap()
        ),
    );

    // verificar si "code" está en el path
    let output = which("code");

    if output.is_ok() {
        // Preguntar si desea instalar plugins para Visual Studio Code para Java y Spring Boot
        let ans = Question::new(
            "\n\n¿Desea instalar plugins para Visual Studio Code para Java y Spring Boot?",
        )
        .acceptable(vec!["s", "n"])
        .default(Answer::YES)
        .show_defaults()
        .until_acceptable()
        .ask();

        if let Some(Answer::YES) = ans {
            // Instalar plugins para Visual Studio Code para Java y Spring Boot
            println!("\nInstalando plugins para Visual Studio Code para Java y Spring Boot...");

            // install vscjava.vscode-spring-initializr and vscjava.vscode-java-pack
            let install_command = "code --install-extension vscjava.vscode-spring-initializr --install-extension vscjava.vscode-java-pack";

            let mut process = Command::new("powershell")
                .stderr(Stdio::piped())
                .arg(install_command)
                .spawn()
                .expect("Error al ejecutar el comando de instalación de plugins para Visual Studio Code para Java y Spring Boot");

            process.wait().expect("Error al correr el comando");
        }
    }

    // Pregunto si desea instalar IntelliJ IDEA Community Edition
    let ans = Question::new("\n\n¿Desea instalar IntelliJ IDEA Community Edition?")
        .acceptable(vec!["s", "n"])
        .default(Answer::YES)
        .show_defaults()
        .until_acceptable()
        .ask();

    if let Some(Answer::YES) = ans {
        // Instalar IntelliJ IDEA Community Edition
        install("intellijidea-community", "IntelliJ IDEA Community Edition");
    }

    // pausa
    let mut input = String::new();
    print!("Presione ENTER para continuar...");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada del usuario");
}

fn check_and_install(cmd: &str, install_cmd: &str, name: &str) {
    println!("\n\nComprobando si {} está instalado...", name);

    let output = which(cmd);

    if output.is_err() {
        // cmd no está instalado, instalarlo
        install(install_cmd, name);
    } else {
        println!("Correcto");
    }
}

fn install(install_cmd: &str, name: &str) {
    println!("\n{} no está instalado, instalando...", name);
    let install_command = format!("choco install -r -y {}", install_cmd);

    let process = Command::new("powershell")
        .stderr(Stdio::piped())
        .arg(install_command)
        .spawn()
        .expect(format!("Error al ejecutar el comando de instalación de {}", name).as_str());

    let out = process
        .wait_with_output()
        .expect("Error al correr el comando");

    if !out.status.success() {
        println!("Error al instalar {}", name);
        // show output
        let stdout = out.stdout;
        let stderr = out.stderr;

        println!("stdout: {}", String::from_utf8_lossy(&stdout));
        println!("stderr: {}", String::from_utf8_lossy(&stderr));
    }
}
