## Docker Image for Imperial First-year Java Labs
A basic alpine based container with `bash`, `java 17`, `checkstyle 8.39` and `google_checks.xml`.

A `lint` script is in the path (checkstyle 8.39 with google config, and giving exit code 1 on warnings).

The google style *currently* used by the department can be found [here](https://static.us.edusercontent.com/files/FcsJmNtP51I8KRe6kHFDgSga).

### Usage
```yml
# example for the Java Functional Programming Lab
image: oliverkillane/java-lab-imperial-2023:latest

stages:
  - build
  - test
  - lint

build:
  stage: build
  script:
    - javac --source-path "src:test" -d out -cp "lib/*" src/advancedstreams/* src/rectangles/* test/*
  artifacts:
    paths:
      - out/

test:
  stage: test
  script:
    - java -cp "lib/hamcrest-all-1.3.jar:lib/junit-4.12.jar:out" org.junit.runner.JUnitCore TestSuite
  dependencies:
    - build

checkstyle:
  stage: lint
  script: 
    - lint src/ test/
  dependencies:
    - build
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
Create a docker repository and replace `oliverkillane` with your own repo.
```bash
docker login # authenticate
docker tag lab_image:latest oliverkillane/java-lab-imperial-2023:latest
docker push oliverkillane/java-lab-imperial-2023:latest
```
