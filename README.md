## Docker Image for Imperial First-year Java Labs
A basic alpine based container with `bash`, `java 17`, `checkstyle 8.39` and `google_checks.xml`.

### Usage
```yml
# todo
```

### Build
```bash
docker build -t lab_image .
```

### Test
To play with the container in bash:
```bash
# Open container in bash shell
# --rm   Remove the container on exit
# -i     Interactive (Keep STDIN open even if not attached)
# -t     Allocate a pseudo-TTY
docker run --rm -it lab_image
```

### Push
Create a docker repository.
```bash
docker login # authenticate
docker tag lab_image:latest oliverkillane/java-lab-imperial-2023:latest
docker push oliverkillane/java-lab-imperial-2023:latest
```