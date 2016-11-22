Building for React Native

Check out demo project:

    git clone https://github.com/bgrins/fluffy-octo-guacamole

Build datomish:

    lein cljsbuild once release-react-native
    cp target/release-react-native/datomish.js ../DatomishReactNative/
