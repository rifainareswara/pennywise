// Ganti adapter-auto ke adapter-node
import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    compilerOptions: {
        runes: ({ filename }) => (filename.split(/[/\\]/).includes('node_modules') ? undefined : true)
    },
    kit: {
        // Sekarang adapter ini akan menghasilkan folder 'build' yang kompatibel dengan Node.js
        adapter: adapter()
    }
};

export default config;