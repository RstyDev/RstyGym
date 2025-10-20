use std::env;
use crate::backend::application::use_cases::{GetAllBooksUseCase, GetAllUsersUseCase, RegisterFamilyUseCase, RegisterUserUseCase, SaveBookUseCase, UpdateUserUseCase};
use crate::backend::infrastructure::repositories::{SurrealBookRepository, SurrealFamilyRepository, SurrealUserRepository};
use crate::entities::{Bautismo, Estado, EstadoCivil, Familia, Libro, Ministerio, Persona, PrestamoLibro, Servicio, Sexo, TipoPresbitero};
use chrono::Local;
use std::sync::Arc;

pub async fn prefill(repo: Arc<SurrealUserRepository>, book_repo: Arc<SurrealBookRepository>, family_repo: Arc<SurrealFamilyRepository>) {

    let mut personas = vec![];
    personas.push(Persona::new(
        None,
        Some(String::from("121212")),
        String::from("Lucas"),
        String::from("Igarzabal"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Miembro {
            conversion: Local::now().date_naive(),
            servicio: vec![
                Servicio::new(true, Ministerio::Tesoro),
                Servicio::new(false, Ministerio::Sonido),
            ],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Vida Sobrenatural"),
            ),
        },
        Local::now().naive_local().date(),
        vec![]
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("232323")),
        String::from("Rafael"),
        String::from("De Lima"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Presbitero {
            conversion: Local::now().date_naive(),
            servicio: vec![
                Servicio::new(true, Ministerio::Palabra),
                Servicio::new(false, Ministerio::Presbiterado),
            ],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Pentecostal de Misiones"),
            ),
            tipo: TipoPresbitero::Maestro,
        },
        Local::now().naive_local().date(),
        vec![]
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("w98g7sd8")),
        String::from("María José"),
        String::from("Cortés Alarcón"),
        Sexo::Femenino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Miembro {
            conversion: Local::now().date_naive(),
            servicio: vec![Servicio::new(false, Ministerio::Bienvenida)],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Iglesia Reformada Renuevo"),
            ),
        },
        Local::now().naive_local().date(),
        vec![]
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("9r7fg8g76y")),
        String::from("Jordi"),
        String::from("Fajardo"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Soltero,
        Estado::PreMiembro {
            conversion: Local::now().date_naive(),
            bautismo: None,
        },
        Local::now().naive_local().date(),
        vec![]
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("9werg78h")),
        String::from("Luciano"),
        String::from("Suarez"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Diacono {
            conversion: Local::now().date_naive(),
            bautismo: Bautismo::new(
                Default::default(),
                Some(Default::default()),
                String::from("Alguna"),
            ),
            servicio: vec![Servicio::new(true, Ministerio::Bienvenida)],
        },
        Default::default(),
        vec![]
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("9d78f6sd")),
        String::from("Matias"),
        String::from("Díaz"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Soltero,
        Estado::Nuevo,
        Local::now().naive_local().date(),
        vec![]
    ));
    if env::var("ENV").unwrap().eq("qa") {
        personas.push(Persona::new(
            None,
            Some(String::from("232323")),
            String::from("admin"),
            String::from("admin"),
            Sexo::Masculino,
            Local::now().date_naive(),
            EstadoCivil::Soltero,
            Estado::Nuevo,
            Local::now().naive_local().date(),
            vec![]
        ))
    }
    let use_case = RegisterUserUseCase::new(repo.clone());
    let book_use = SaveBookUseCase::new(book_repo.clone());
    for persona in personas {
        use_case.execute(persona).await.unwrap();
    }
    let users = GetAllUsersUseCase::new(repo.clone()).get_all().await.unwrap();

    let mut lucas = Persona::default();
    let mut majo = Persona::default();
    for user in users {
        if user.nombre().eq("Lucas") {
            lucas = user;
        } else if user.nombre().eq("María José") {
            majo = user;
        }
    }
    let familia = Familia::new(None,String::from("Igarzabal Cortés"),Some(lucas.clone()),Some(majo),vec![]);
    let family_use = RegisterFamilyUseCase::new(family_repo);
    family_use.execute(familia).await.unwrap();
    let libro = Libro::new(None,String::from("Conocer a Dios"),String::from("J.I. Packer"),String::from("978-1-955182-01-0"),String::from("Poiema"),1890,2023,355,PrestamoLibro::Usuario {id:lucas.id().unwrap().to_string(),dias:10,fecha:Local::now().date_naive()});
    let libro2 = Libro::new(None, String::from("Asombrados Por Dios"),String::from("John Piper"),String::from("978-1-5359-5716-8"),String::from("B&H Publishing Group"),2018,2019,165,PrestamoLibro::None);
    book_use.execute(libro.clone()).await.unwrap();
    book_use.execute(libro2).await.unwrap();
    lucas.set_password(Some(String::from("121212")));
    // lucas.add_libro(libro);
    let libros = GetAllBooksUseCase::new(book_repo).get_all().await.unwrap();

    let update_use = UpdateUserUseCase::new(repo);
    update_use.add_book(lucas.id().unwrap().as_str(),libros[0].clone()).await.unwrap();

}
