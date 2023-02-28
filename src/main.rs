use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    // Comprobar si Chocolatey está instalado

    println!("Comprobando si Chocolatey está instalado...");
    let output = Command::new("powershell")
        .arg("Get-Command choco")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if !output.status.success() {
        // Chocolatey no está instalado, instalarlo
        println!("Chocolatey no está instalado, instalando...");
        let install_command = "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))";

        Command::new("powershell")
            .arg(install_command)
            .output()
            .expect("Error al ejecutar el comando de instalación de Chocolatey");

        // Añadir Chocolatey al PATH
        env::set_var("PATH", format!("{};C:\\ProgramData\\chocolatey\\bin", env::var("PATH").unwrap()));
    }

    // Comprobar si OpenJDK 17 está instalado
    println!("Comprobando si OpenJDK 17 está instalado...");
    let output = Command::new("powershell")
        .arg("Get-Command java")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if !output.status.success() {
        // OpenJDK 17 no está instalado, instalarlo
        println!("OpenJDK 17 no está instalado, instalando...");
        let install_command = "choco install -y openjdk17 --version=17.0.2";

        Command::new("powershell")
            .arg(install_command)
            .output()
            .expect("Error al ejecutar el comando de instalación de OpenJDK");
    }

    // Comprobar si Maven está instalado

    println!("Comprobando si Maven está instalado...");

    let output = Command::new("powershell")
        .arg("Get-Command mvn")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if !output.status.success() {
        // Maven no está instalado, instalarlo
        println!("Maven no está instalado, instalando...");
        let install_command = "choco install -y maven";

        Command::new("powershell")
            .arg(install_command)
            .output()
            .expect("Error al ejecutar el comando de instalación de Maven");
    }

    // Comprobar si Postgres está instalado

    println!("Comprobando si Postgres está instalado...");

    let output = Command::new("powershell")
        .arg("Get-Command psql")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if !output.status.success() {
        // use gui installer from https://sbp.enterprisedb.com/getfile.jsp?fileid=1258323 with reqwest
        // Postgres no está instalado, instalarlo
        println!("Postgres no está instalado, instalando...");
        let target = "https://sbp.enterprisedb.com/getfile.jsp?fileid=1258323";
        let mut resp = reqwest::blocking::get(target).unwrap();
        let mut file = std::fs::File::create("postgres.exe").unwrap();
        std::io::copy(&mut resp, &mut file).unwrap();

        file.flush().unwrap();
        file.sync_all().unwrap();
        //stop file being used
        drop(file);

        Command::new("postgres.exe")
            .output()
            .expect("Error al ejecutar el comando de instalación de Postgres");

    }

    // Comprobar si Git está instalado

    println!("Comprobando si Git está instalado...");

    let output = Command::new("powershell")
        .arg("Get-Command git")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if !output.status.success() {
        // Git no está instalado, instalarlo
        println!("Git no está instalado, instalando...");
        let install_command = "choco install -y git";

        Command::new("powershell")
            .arg(install_command)
            .output()
            .expect("Error al ejecutar el comando de instalación de Git");
    }

    // Preguntar si desea instalar Visual Studio Code
    let mut input = String::new();
    print!("¿Desea instalar Visual Studio Code? (s/n) ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Error al leer la entrada del usuario");

    if input.trim().to_lowercase() == "s" {
        // Instalar Visual Studio Code
        println!("Instalando Visual Studio Code...");
        let install_command = "choco install -y vscode";

        Command::new("powershell")
            .arg(install_command)
            .output()
            .expect("Error al ejecutar el comando de instalación de Visual Studio Code");
    }

    // agregar vscode al path
    env::set_var("PATH", format!("{};C:\\Program Files\\Microsoft VS Code\\bin", env::var("PATH").unwrap()));

    // verificar si "code" está en el path
    let output = Command::new("powershell")
        .arg("Get-Command code")
        .output()
        .expect("Error al ejecutar el comando 'powershell'");

    if output.status.success() {
        // Preguntar si desea instalar plugins para Visual Studio Code para Java y Spring Boot
        let mut input = String::new();
        print!("¿Desea instalar plugins para Visual Studio Code para Java y Spring Boot? (s/n) ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Error al leer la entrada del usuario");

        if input.trim().to_lowercase() == "s" {
            // Instalar plugins para Visual Studio Code para Java y Spring Boot
            println!("Instalando plugins para Visual Studio Code para Java y Spring Boot...");

            // install vscjava.vscode-spring-initializr and vscjava.vscode-java-pack
            let install_command = "code --install-extension vscjava.vscode-spring-initializr --install-extension vscjava.vscode-java-pack";

            Command::new("powershell")
                .arg(install_command)
                .output()
                .expect("Error al ejecutar el comando de instalación de plugins para Visual Studio Code para Java y Spring Boot");
        }
    }

    // pause the console
    let mut input = String::new();
    print!("Presione ENTER para continuar...");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Error al leer la entrada del usuario");
}
