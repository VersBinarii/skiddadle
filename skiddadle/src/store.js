import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
	policy: null,
	// bucket: 'http://powerofkek.s3-website.eu-central-1.amazonaws.com'
	bucket: 'https://s3.eu-central-1.amazonaws.com/powerofkek',
	meme: ''
    },
    mutations: {
	GET_S3_POLICY_SUCCESS(state, data) {
	    state.policy = data.body.policy;
	},
	MEME_GENERATION_SUCCESS(state, meme) {
	    state.meme = meme;
	}
    },
    actions: {
	async fetchS3Policy({
	    commit
	}, payload) {
	    try {
		const {
		    data
		} = await axios.get(`https://6ukgq70no1.execute-api.eu-central-1.amazonaws.com/policy/sign-s3-request?filename=${payload.name}`);
		commit('GET_S3_POLICY_SUCCESS', data);
	    } catch (err) {
		console.log(err);
	    }
	},
	async uploadToS3({
	    state,
	    commit
	}, payload) {
	    try {
		let {
		    data
		} = await axios.post(`${state.bucket}`, payload);
	    } catch (err) {
		console.log(err);
	    }
	},
	async submitToLambda({
	    commit
	}, payload) {
	    try {
		const {
		    data
		} = await axios
		      .post('https://nrr7fjnt8b.execute-api.eu-central-1.amazonaws.com/production/meme',
			    payload);
		console.log(data);
        commit('MEME_GENERATION_SUCCESS', data);
      } catch (err) {
        console.log(err);
      }
    }
  }
});
