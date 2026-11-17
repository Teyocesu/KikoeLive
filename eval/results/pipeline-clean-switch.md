# Pipeline run: `eval/data/tts/clean.wav`

- corpus CER (ASR vs truth): 0.117
- mean chrF en: Some(64.9), es: Some(48.9)
- speech start -> subtitle (en): n=29 median 2898 / p95 3795 / max 4044
- speech end -> subtitle (en): n=29 median 1125 / p95 1625 / max 2370
- speech start -> subtitle (es): n=16 median 2445 / p95 3730 / max 3774
- speech end -> subtitle (es): n=16 median 1127 / p95 1382 / max 1644
- ASR ms: n=49 median 329 / p95 403 / max 455
- MT ms en: n=30 median 269 / p95 491 / max 689 | es: n=19 median 250 / p95 454 / max 668
- MT queue wait en: n=30 median 0 / p95 0 / max 0 | es: n=19 median 0 / p95 0 / max 0
- segments 49 (forced cuts 0, blips 0), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 0 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 49, translations: 49, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: ["LANGUAGE_SWITCH es", "LANGUAGE_SWITCH en"]
- stop: 44 ms (joined true)
- capture: CaptureStats { packets: 12572, frames: 6034268, dropped_packets: 0, discontinuities: 0, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよ。 | 0.18 | en | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100 | 3795 | 1625 |
| c02 | え、待って、それ本当？ | え、待って、それ本当。 | 0.00 | en | Wait, what? Is that true? | Wait, really? | 18 | 2976 | 1036 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい、死ぬ死ぬ。 | 0.00 | en | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die. | 69 | 3035 | 1325 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | en | It's not scary at all. | It's not scary at all. | 100 | 2192 | 872 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど、大丈夫。 | 0.00 | en | It's not that I'm not scared, but I'm okay. | It's not exactly scary, but I'm okay. | 50 | 3511 | 1301 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで、ちょっと待って。 | 0.00 | en | Don't go yet, wait a second. | Don't go yet, wait a little. | 67 | 2779 | 1199 |
| c07 | みんな、コメントありがとう！ | みんな、コメントありがとう。 | 0.00 | en | Thanks for the comments, everyone! | Thanks for all the comments, everyone! | 89 | 3003 | 1123 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ | 0.12 | en | Today we're going to play a new game, peko. | Today, Peko will play a new game! | 47 | 3162 | 1202 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長のせいだよ。 | 0.00 | en | That's Captain Marine's fault. | That's Captain Marine's fault. | 100 | 2595 | 1125 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの? | 0.00 | en | Who said that? | Who said that? | 100 | 2422 | 932 |
| c11 | もう二度とやらない。 | もう二度とやらない。 | 0.00 | en | I'm never doing this again. | I won't do it again. | 26 | 1951 | 1041 |
| c12 | あれ、ドアが開かない。 | あれ、ドアが開かない。 | 0.00 | en | Huh, the door won't open. | Wait, the door won't open. | 83 | 2393 | 1053 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう。 | 0.00 | en | Let's get the key and then go back. | I'll go back and get the key. | 46 | 2522 | 1002 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん。 | 0.00 | en | I told you it was left! | I told you to go left! | 51 | 2054 | 964 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | en | Isn't the sound a bit too loud? | Wait, is the volume too loud? | 34 | 2300 | 1170 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | es | Perdón por empezar tarde la transmisión. | Disculpa, empecé el stream tarde. | 21 | 3774 | 1644 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたから、ちょっと休憩する。 | 0.00 | es | Tengo hambre, así que voy a tomar un descansito. | ¡Tengo hambre, voy a hacer una pequeña pausa! | 37 | 3627 | 1307 |
| c18 | これ、絶対罠でしょ。 | これ、絶対罠でしょ。 | 0.00 | es | Esto seguro que es una trampa, ¿no? | ¡Esto, seguro que es una trampa! | 74 | 2578 | 1178 |
| c19 | 勝った！やったー！ | 勝った。やったー。 | 0.00 | es | ¡Gané! ¡Bien! | ¡Ganamos! ¡Qué bien! | 40 | 1339 | -351 |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど。 | 0.00 | es | Es que no quiero perder. | No quería perder. | 40 | 2202 | 1112 |
| c21 | 知らなかった、そんなの。 | 知らなかった、そんなの。 | 0.00 | es | No sabía eso. | ¡No lo sabía, eso! | 47 | 2445 | 1035 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ。 | 0.33 | es | Eso no es de Pekora. | Eso no es Pekora. | 66 | 2689 | 1179 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから、見に来てね。 | 0.00 | es | Mañana también voy a transmitir, así que vengan a ver. | Mañana también haré una transmisión, así que ven a verla. | 61 | 3652 | 1382 |
| c24 | え、なんで？意味わかんない。 | え、なんで?意味わかんない | 0.00 | es | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No tengo idea de qué significa. | 42 | 1780 | -300 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直しだ。 | 0.00 | es | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si muero aquí, tengo que empezar de nuevo. | 70 | 3730 | 1309 |
| c26 | 先輩に怒られちゃった。 | 先輩に怒られちゃった。 | 0.00 | es | Mi senpai me regañó. | Me enojó el senpai. | 29 | 2429 | 1079 |
| c27 | 別に好きじゃないし。 | 別に好きじゃないし。 | 0.00 | es | Tampoco es que me guste. | No es que me guste. | 69 | 2018 | 1008 |
| c28 | お疲れぺこ！また明日！ | お疲れペコ。また明日。 | 0.22 | es | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias por la atención, Pekora! ¡Mañana otra vez. | 29 | 2001 | -399 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | es | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63 | 2347 | 1127 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない。 | 0.10 | es | Todavía no he ganado ni una vez. | Aún no he comprado una sola vez. | 27 | 2157 | 1097 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう。助かる。 | 0.00 | es | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat! Me alivia. | 63 | 3356 | 1296 |
| c32 | あ、そこ宝箱あるじゃん。 | あ、そこ宝箱あるじゃん。 | 0.00 | en | Oh, there's a treasure chest right there. | Oh, there's a treasure chest right there. | 100 | 2937 | 1237 |
| c33 | 行けると思ったのに。 | いけると思ったのに。 | 0.11 | en | I thought I could make it... | I thought I could go. | 60 | 1943 | 993 |
| c34 | 嘘でしょ、また落ちた。 | 嘘でしょ。また落ちた。 | 0.00 | en | No way, I fell again. | That's a lie. I fell again. | 55 | 2945 | 1225 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないって。マイク大丈夫 | 0.00 | en | You can't hear my voice? Is the mic okay? | You can't hear the voice. The mic is fine. | 51 | 2112 | -588 |
| c36 | 笑わないでよ！ | 笑わないでよ。 | 0.00 | en | Don't laugh! | Don't laugh at me. | 80 | 1682 | 982 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です | 0.00 | en | I'm part of hololive's third generation. | I'm a third-generation hololive member. | 51 | 2732 | 1152 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて。 | 0.00 | en | I won't mess up next time, so watch. | Next time I won't fail, so watch this. | 54 | 2965 | 1355 |
| c39 | この敵、倒さなくてもいいよ。 | この敵、倒さなくてもいいよ。 | 0.00 | en | You don't have to defeat this enemy. | This enemy doesn't have to be defeated. | 60 | 2898 | 1138 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっけ? | 0.00 | en | Um, what were we talking about? | Um, what were we talking about? | 100 | 3061 | 1121 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | en | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday. | 62 | 3270 | 1190 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | めっちゃ楽しかったこのゲーム前にもやったことあるんだよね | 0.47 | en | I've actually played this game before. | That game was super fun, and I've played it before. | 38 | 4044 | 1114 |
| p2b | でも全然覚えてない。 |  | 1.00 | - |  |  | 0 | - | - |
| p3a | 犯人はたぶんあの人だと思う。 | でも全然覚えてない。犯人は多分あの人だと思う。 | 0.92 | en | I think the culprit is probably that person. | But I don't remember anything at all. I think the culprit is probably that person. | 85 | 3035 | 1505 |
| p3b | いや、違うかも。 | いや、違うかも。宿題あった。 | 0.83 | en | No, maybe not. | No, maybe not. I had homework. | 78 | 3700 | 2370 |
| p4a | 宿題やった？ |  | 1.00 | - |  |  | 0 | - | - |
| p4b | まだやってない。 | まだやってない。 | 0.00 | en | I haven't done it yet. | Not done yet. | 28 | 1594 | 954 |
