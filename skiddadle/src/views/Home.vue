<template>
  <v-card class="mx-auto" max-width="800">
    <v-toolbar card dense>
      <v-toolbar-title>
        <span class="subheading">Memememe</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn icon>
        <v-icon>share</v-icon>
      </v-btn>
    </v-toolbar>
    <v-container fill-height style="min-height: 434px" fluid>
      <v-layout v-if="show" key="0">
        <v-flex>
          <v-img :src="theImage" class="grey darken-4"></v-img>
        </v-flex>
      </v-layout>

      <v-layout v-else key="1" justify-center>
        <v-btn @click="showImageSelect">Load image</v-btn>
        <input
          type="file"
          style="display: none"
          ref="imgUpload"
          accept="image/*"
          @change="onFilePicked"
        >
      </v-layout>
    </v-container>
    <v-card-actions>
      <v-layout wrap>
        <v-flex xs12 d-flex>
          <v-flex xs3>
            <v-subheader>Caption</v-subheader>
          </v-flex>
          <v-flex>
            <v-text-field v-model="caption" placeholder="Enter caption"></v-text-field>
          </v-flex>
        </v-flex>
        <v-flex xs12 d-flex>
          <v-flex xs3>
            <v-subheader>Position X</v-subheader>
          </v-flex>
          <v-flex>
            <v-slider
              v-model="posx"
              always-dirty
              :max="imageWidth"
              :color="color"
              thumb-label="always"
              :thumb-size="thumbsize"
              @change="upload()"
            ></v-slider>
          </v-flex>
        </v-flex>

        <v-flex xs12 d-flex>
          <v-flex xs3>
            <v-subheader>Position Y</v-subheader>
          </v-flex>
          <v-flex>
            <v-slider
              v-model="posy"
              always-dirty
              :max="imageHeight"
              :color="color"
              thumb-label="always"
              :thumb-size="thumbsize"
              @change="upload()"
            ></v-slider>
          </v-flex>
        </v-flex>

        <v-flex xs12 d-flex>
          <v-flex xs3>
            <v-subheader>Scale</v-subheader>
          </v-flex>
          <v-flex>
            <v-slider
              v-model="scale"
              always-dirty
              min="30"
              max="200"
              :color="color"
              thumb-label="always"
              :thumb-size="thumbsize"
              @change="upload()"
            ></v-slider>
          </v-flex>
        </v-flex>
        <v-flex>
          <v-btn @click="upload()">Upload</v-btn>
          <v-btn @click="clearAll()">Clear</v-btn>
        </v-flex>
      </v-layout>
    </v-card-actions>
  </v-card>
</template>


<script>
  export default {
    data: () => ({
      posx: 0,
      posy: 0,
      scale: 30,
      caption: '',
      show: false,
      thumbsize: 25,
      imageName: '',
      imageWidth: 0,
      imageHeight: 0,
      file: null,
      theImage: '',
    }),
    methods: {
      clearAll() {
        this.show = false;
        this.imageName = '';
        this.posx = 0;
        this.posy = 0;
        this.caption = '';
        this.scale = 30;
        this.imageHeight = 0;
        this.imageWidth = 0;
      },
      showImageSelect() {
        this.$refs.imgUpload.click();
      },
      onFilePicked(e) {
        this.file = e.target.files[0];
        this.$store
          .dispatch('fetchS3Policy', {
            name: this.file.name,
            type: this.file.type,
          })
          .then(() => {
            const policy = this.$store.state.policy;
            const formData = new FormData();
            formData.append('key', policy.key);
            formData.append('acl', policy.acl);
            formData.append('Content-Type', this.file.type);
            formData.append('policy', policy.base64Policy);
            formData.append('x-amz-credential', policy['x-amz-credential']);
            formData.append('x-amz-algorithm', policy['x-amz-algorithm']);
            formData.append('x-amz-date', policy['x-amz-date']);
            formData.append('x-amz-signature', policy['x-amz-signature']);
            formData.append('x-amz-server-side-encryption', 'AES256');
            formData.append('file', this.file);
            this.$store.dispatch('uploadToS3', formData).then(() => {
              this.imageName = this.file.name;
              const image = new Image();
              image.src = this.getImage;
              image.addEventListener('load', () => {
                this.theImage = this.getImage;
                this.show = true;
                this.imageWidth = image.width;
                this.imageHeight = image.height;
              });
            });
          })
          .catch(err => {
            console.log(err);
          });
      },
      async submitAll(formData) {
        await this.$store.dispatch('submitToLambda', formData);
      },
      async upload() {
        const formData = {};
        formData['image'] = this.$store.state.policy.key;
        formData['bucket_address'] = this.$store.state.bucket;
        formData['posx'] = this.posx;
        formData['posy'] = this.posy;
        formData['scale'] = this.scale;
        formData['caption'] = this.caption;
        await this.submitAll(formData);
	console.log(this.$store.state.meme);
        this.theImage = `data:${this.$store.state.meme.meme_type};base64,${this.$store.state.meme.meme_data}`;
      }
    },
    computed: {
      color() {
        return 'red';
      },
      getImage() {
        return `${this.$store.state.bucket}/static/${this.imageName}`;
      },
    },
  };
</script>
