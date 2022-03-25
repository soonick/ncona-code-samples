java_binary(
  name = 'hello_world',
  srcs = glob(['*.java']),
  classpath_resources = [
    'logback.xml'
  ],
  main_class = 'cadence.helloworld.GettingStarted',
  deps = [
    '@maven//:com_uber_cadence_cadence_client',
    '@maven//:commons_configuration_commons_configuration',
    '@maven//:ch_qos_logback_logback_classic',
    '@maven//:org_slf4j_slf4j_api'
  ],
)
