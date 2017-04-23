# java文件操作

主要是记录java项目使用maven的构建

### 基本环境构建
1. 查找自己要使用的一些组建
2. 在idea中配置自己的pom.xml文件，配置需要的第三方库。
3. 使用maven添加依赖第三方库[第三方库地址](https://mvnrepository.com/)
4. 写基本的java代码，编译运行


想要将所有的jar打包成一个jar，方便发布和运行。
但是出现问题：*xxxx.jar中没有主清单属性*

### 代码打包成jar
1.在pom.xml中添加assembly 插件，并且明确maifest, mainClass为你之前已经能运行的java入口，在当前项目为PdfRead
```xml

<build>
    <plugins>
        <plugin>
            <artifactId>maven-assembly-plugin</artifactId>
            <version>2.4</version>
            <configuration>
                <descriptorRefs>
                    <descriptorRef>jar-with-dependencies</descriptorRef>
                </descriptorRefs>
                <archive>
                    <manifest>
                        <mainClass>PdfRead</mainClass>
                    </manifest>
                </archive>
            </configuration>
            <executions>
                <execution>
                    <id>make-assembly</id>
                    <phase>package</phase>
                    <goals>
                        <goal>single</goal>
                    </goals>
                </execution>
            </executions>
        </plugin>
    </build>
</plugins>
```


2.运行 maven，加载插件assembly，运行插件assembly，在target中输出目标jar。然后在cmd中运行：
```shell
java -jar *.jar
```

3.添加配置配置文件
添加 *properties* 配置文件, 既然是配置文件，那么必然会在服务器和开发环境 不同，
所以需要能够修改，并且不用编译源码
在idea中，`properties` 文件放到 `src/main/resources/`下。使用相对路径来访问properties文件。

```java
      public PropertiesUtils(String filePath) {
               propertie = new Properties();
               try {
                   inputFile = new FileInputStream(this.getClass().getClassLoader()
                           .getResource(filePath).getPath());
                   System.out.println("inputFile" + inputFile);

                   propertie.load(inputFile);
                   inputFile.close();
               } catch (FileNotFoundException ex) {
                   System.out.println("读取属性文件--->失败！- 原因：文件路径错误或者文件不存在");
                   ex.printStackTrace();

   //                String s_config="resources/db.properties";
                   String s_config="F:\\opensource\\some_lang\\java\\pdf_parse\\target\\resources\\" + filePath;

                   try {
                       FileInputStream in = new FileInputStream(s_config);
                       propertie.load(in);
                   } catch (FileNotFoundException s_ex) {
                       s_ex.printStackTrace();
                       System.out.println(" 打开 " + s_config + "失败！");
                   } catch (IOException s_ex) {
                       ex.printStackTrace();
                   }



               } catch (IOException ex) {
                   System.out.println("装载文件--->失败!");
                   ex.printStackTrace();
               }
           }// end ReadConfigInfo(...)
```

4.这个时候使用 ```assembly:assembly``` 编译出来的文件找不到，properties文件，
这个时候，我们使用绝对路径来访问properties文件。也就是上边代码中的绝对地址。
当然，这个在开发环境中，一般不会用到，所以我们可以直接改成生产环境的绝对路径
这个时候，需要继续配置pom.xml文件,添加插件resources，设置你的配置文件jar之外的目录```outputDirectory```
```xml

    <build>
        <plugins>
            <plugin>
                <artifactId>maven-assembly-plugin</artifactId>
                <version>2.4</version>
                <configuration>
                    <descriptorRefs>
                        <descriptorRef>jar-with-dependencies</descriptorRef>
                    </descriptorRefs>
                    <archive>
                        <manifest>
                            <mainClass>PdfRead</mainClass>
                        </manifest>
                    </archive>
                </configuration>
                <executions>
                    <execution>
                        <id>make-assembly</id>
                        <phase>package</phase>
                        <goals>
                            <goal>single</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
            <plugin>
                <artifactId>maven-resources-plugin</artifactId>
                <executions>
                    <execution>
                        <id>copy-resources</id>
                        <phase>validate</phase>
                        <goals>
                            <goal>copy-resources</goal>
                        </goals>
                        <configuration>
                            <outputDirectory>${project.build.directory}/resources</outputDirectory>
                            <resources>
                                <resource>
                                    <directory>src/main/resources</directory>
                                    <filtering>true</filtering>
                                </resource>
                            </resources>
                        </configuration>

                    </execution>
                </executions>
            </plugin>
        </plugins>

    </build>


```

项目基本搭建完成。
