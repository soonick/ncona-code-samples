java_binary(
  name = 'di_example',
  srcs = glob(['*.java']),
  classpath_resources = [
    'beans.xml'
  ],
  main_class = 'diexample.DiExample',
  deps = [
    '@maven//:org_springframework_spring_context',
  ],
)
