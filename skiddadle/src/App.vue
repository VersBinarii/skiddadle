<template>
  <v-app>
    <v-content>
      <router-view />
    </v-content>
    <v-footer
      :fixed="fixed"
      app
    >
      <v-spacer></v-spacer>
      <span>&copy; 2017</span>
      <v-spacer></v-spacer>
    </v-footer>
  </v-app>
</template>

<script>
  export default {
    data() {
      return {
        fixed: false,
        title: "Skiddadle"
      };
    },
    name: "App"
  };
</script>
