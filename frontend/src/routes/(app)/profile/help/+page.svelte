<script lang="ts">
  import Navbar from '$lib/components/Navbar.svelte';
  import { slide } from 'svelte/transition';
  import { version } from '$app/environment';

  let activeAccordion = $state<number | null>(null);

  const faqs = [
    {
      q: 'Bagaimana cara menambah transaksi baru?',
      a: 'Anda dapat menekan ikon `+` di bilah Beranda (Home) atau Dompet, lalu masukkan nominal uang, kategori, dan deskripsi transaksinya.'
    },
    {
      q: 'Apa itu Smart Task List?',
      a: 'Ini adalah fitur revolusioner yang dapat mengelola prioritas dan mengingatkan Anda berbagai hal mulai dari daftar Belanjaan, mencatat Hutang/Piutang otomatis, sampai Tagihan harian.'
    },
    {
      q: 'Apakah data keuangan saya aman?',
      a: 'Seluruh nominal uang dan *password* Anda di-enkripsi di peladen pribadi. Kami tidak akan pernah memanfaatkan apalagi menjual data historis transaksi ke tangan ketiga.'
    },
    {
      q: 'Bagaimana saya menghubungkan rekening Bank asli?',
      a: 'Saat ini PennyWise V1 berperan sebagai pelacak dompet manual (*Tracker*), fungsi integrasi Open-Banking otomatis sedang dalam tahap beta pengembangan.'
    }
  ];

  function toggleFaq(index: number) {
    activeAccordion = activeAccordion === index ? null : index;
  }
</script>

<svelte:head>
  <title>Bantuan — PennyWise</title>
</svelte:head>

<Navbar title="Bantuan & Dukungan" showBack={true} />

<main class="px-6 pt-4 max-w-lg mx-auto pb-20 page-transition">
  <div class="mt-6 mb-8">
    <h2 class="font-title text-2xl mb-1 text-on-background">Ada yang bisa dibantu?</h2>
    <p class="text-sm font-medium text-on-surface-variant">Temukan jawaban untuk pertanyaan umum di bawah ini.</p>
  </div>

  <section class="flex flex-col gap-3">
    {#each faqs as faq, i}
      <button 
        class="w-full text-left bg-surface-container-high rounded-xl p-4 transition-colors cursor-pointer group outline-none"
        onclick={() => toggleFaq(i)}
      >
        <div class="flex justify-between items-center gap-4">
          <h3 class="font-bold text-on-surface text-[15px]">{faq.q}</h3>
          <span class="material-symbols-outlined text-primary transition-transform duration-300 {activeAccordion === i ? 'rotate-180' : ''}">
            expand_more
          </span>
        </div>
        {#if activeAccordion === i}
          <div transition:slide={{ duration: 300 }}>
            <p class="mt-3 text-sm text-on-surface-variant leading-relaxed pb-1 border-t border-outline/10 pt-3">
              {faq.a}
            </p>
          </div>
        {/if}
      </button>
    {/each}
  </section>

  <div class="mt-10 p-5 bg-surface-container-highest rounded-[20px] text-center flex flex-col items-center relative overflow-hidden">
    <span class="material-symbols-outlined text-[36px] text-on-surface-variant mb-2">support_agent</span>
    <h3 class="font-bold text-on-surface mb-1">Masih Mengalami Kendala?</h3>
    <p class="text-xs text-on-surface-variant font-medium leading-relaxed max-w-[250px] mb-4">
      Tim pengembang kami bersedia membantu via pesan Telegram jika pertanyaan Anda belum terjawab di sini.
    </p>
    <a href="https://github.com" target="_blank" class="px-6 py-2.5 bg-on-surface text-surface rounded-full text-sm font-bold active:scale-95 transition-transform flex items-center gap-2">
      <span class="material-symbols-outlined text-[18px]">open_in_new</span>
      Ajukan Isu Bantuan
    </a>
  </div>

  <div class="mt-8 text-center">
    <p class="text-[11px] uppercase tracking-widest text-on-surface-variant/40 font-label font-medium">
      PennyWise App Build
      <br>
      <span class="text-on-surface-variant font-bold text-xs">Version {version}</span>
    </p>
  </div>
</main>
