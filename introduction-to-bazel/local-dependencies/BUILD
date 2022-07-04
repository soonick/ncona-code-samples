java_binary(
  name = 'example_package',
  srcs = ['ExamplePackage.java'],
  main_class = 'example.ExamplePackage',
  deps = [':util'],
)

java_library(
  name = 'util',
  srcs = glob(['util/*.java']),
)
