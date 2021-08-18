# CLI OhMy Mockserver Bridge
Implementación de un CLI que permite cargar un MockServer a partir de archivos exportados por plugin de chrome oh-my-mock

## Corriendo MockServer en Docker
Se utilizará la imagen [Docker oficial](https://hub.docker.com/r/mockserver/mockserver) de [MockServer](https://www.mock-server.com/). Para mayor información puedes revisar la [documentación oficial](https://www.mock-server.com/where/docker.html#pull_docker_image).

### Descarga de la imágen
```
docker pull mockserver/mockserver
```

### Correr imágen Docker
Este comando correra una nueva instancia con la configuración por defecto y sin persistir la *'expectations'* que se hayan creado.

Al parar la instancia, el contenedor será eliminado.
```
docker run -p 1080:1080 --rm mockserver/mockserver:mockserver-5.11.2
```

## Ejecutando CLI Bridge
A continuación se explica como ejecutar el CLI para la creación o actualización de las *'expectations'* en MockServer. 