estimated playback offset: 2260 ms

# Pipeline run: `live system audio`

- corpus CER (ASR vs truth): 0.123
- mean chrF en: Some(61.5), es: Some(50.8)
- speech start -> subtitle (en): n=21 median 4159 / p95 11953 / max 13918
- speech end -> subtitle (en): n=21 median 2717 / p95 10013 / max 11748
- speech start -> subtitle (es): n=23 median 4531 / p95 8946 / max 12927
- speech end -> subtitle (es): n=23 median 3091 / p95 6525 / max 11417
- ASR ms: n=49 median 272 / p95 536 / max 681
- MT ms en: n=22 median 1266 / p95 1988 / max 10171 | es: n=26 median 1438 / p95 2203 / max 10170
- MT queue wait en: n=22 median 0 / p95 5977 / max 7458 | es: n=26 median 0 / p95 4948 / max 4979
- segments 49 (forced cuts 0, blips 0), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 1 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 49, translations: 48, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: ["LANGUAGE_SWITCH es"]
- stop: 771 ms (joined true)
- capture: CaptureStats { packets: 14094, frames: 6765120, dropped_packets: 0, discontinuities: 1, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよ | 0.18 | en | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100 | 13918 | 11748 |
| c02 | え、待って、それ本当？ | え、待って、それ本当。 | 0.00 | en | Wait, what? Is that true? | Wait, really? | 18 | 11953 | 10013 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい、死ぬ死ぬ。 | 0.00 | en | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die. | 69 | 10789 | 9079 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | en | It's not scary at all. | It's not scary at all. | 100 | 9098 | 7778 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど、大丈夫。 | 0.00 | en | It's not that I'm not scared, but I'm okay. | It's not exactly scary, but I'm okay. | 50 | 7850 | 5640 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで、ちょっと待って。 | 0.00 | en | Don't go yet, wait a second. | Don't go yet, wait a little. | 67 | 5213 | 3633 |
| c07 | みんな、コメントありがとう！ | みんな、コメントありがとう。 | 0.00 | en | Thanks for the comments, everyone! | Thanks for all the comments, everyone! | 89 | 3694 | 1814 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ | 0.12 | en | Today we're going to play a new game, peko. | Today, Peko will play a new game! | 47 | 4963 | 3003 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長のせいだよ。 | 0.00 | en | That's Captain Marine's fault. | That's Captain Marine's fault. | 100 | 4061 | 2591 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの。 | 0.00 | en | Who said that? | Who said that? | 100 | 3749 | 2259 |
| c11 | もう二度とやらない。 | もう二度とやらない。 | 0.00 | en | I'm never doing this again. | I won't do it again. | 26 | 3371 | 2461 |
| c12 | あれ、ドアが開かない。 | あれ、ドアが開かない。 | 0.00 | en | Huh, the door won't open. | Wait, the door won't open. | 83 | 3901 | 2561 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう。 | 0.00 | en | Let's get the key and then go back. | I'll go back and get the key. | 46 | 4237 | 2717 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん。 | 0.00 | en | I told you it was left! | I told you to go left! | 51 | 3552 | 2462 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | en | Isn't the sound a bit too loud? | Wait, is the volume too loud? | 34 | 3701 | 2571 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | en | Sorry for starting the stream late. | Sorry, I started the stream a bit late. | 49 | 4856 | 2726 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたから、ちょっと休憩する。 | 0.00 | en | I'm hungry, so I'll take a short break. | My stomach's rumbling, so I'm taking a short break. | 47 | 5592 | 3272 |
| c18 | これ、絶対罠でしょ。 | これ、絶対罠でしょ。 | 0.00 | en | This is definitely a trap, right? | This has to be a trap, right? | 47 | 4159 | 2759 |
| c19 | 勝った！やったー！ | 勝ったやったー。 | 0.00 | en | I won! Yay! | I won Got it! | 22 | 2290 | 600 |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど。 | 0.00 | en | I really don't want to lose. | I don't want to lose. | 70 | 3435 | 2345 |
| c21 | 知らなかった、そんなの。 | 知らなかった、そんなの。 | 0.00 | en | I didn't know that. | Didn't know that, that stuff. | 76 | 3917 | 2507 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ。 | 0.33 | es | Eso no es de Pekora. | Eso no es Pekora. | 66 | 12927 | 11417 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから、見に来てね。 | 0.00 | - |  | falling behind: newer line arrived | 0 | - | - |
| c24 | え、なんで？意味わかんない。 | え、なんで?意味わかんない | 0.00 | es | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No tengo idea de qué significa. | 42 | 8327 | 6247 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直しだ。 | 0.00 | es | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si muero aquí, tengo que empezar de nuevo. | 70 | 8946 | 6525 |
| c26 | 先輩に怒られちゃった。 | 先輩に怒られちゃった。 | 0.00 | es | Mi senpai me regañó. | Me enojó el senpai. | 29 | 7392 | 6042 |
| c27 | 別に好きじゃないし。 | 別に好きじゃないし。 | 0.00 | es | Tampoco es que me guste. | No es que me guste. | 69 | 6672 | 5662 |
| c28 | お疲れぺこ！また明日！ | お疲れペコ。また明日。 | 0.22 | es | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias por la atención, Pekora! ¡Mañana otra vez. | 29 | 6513 | 4113 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | es | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63 | 5276 | 4056 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない。 | 0.10 | es | Todavía no he ganado ni una vez. | Aún no he comprado una sola vez. | 27 | 4470 | 3410 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう、助かる。 | 0.00 | es | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat, me salvó! | 70 | 4228 | 2168 |
| c32 | あ、そこ宝箱あるじゃん。 | あ、そこ宝箱あるじゃん。 | 0.00 | es | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí! | 42 | 3693 | 1993 |
| c33 | 行けると思ったのに。 | いけると思ったのに。 | 0.11 | es | Pensé que lo iba a lograr... | ¡Pensé que podía ir. | 33 | 3283 | 2333 |
| c34 | 嘘でしょ、また落ちた。 | 嘘でしょ。また落ちた。 | 0.00 | es | No puede ser, me caí otra vez. | ¡Miente! ¡Me caí otra vez! | 47 | 4427 | 2707 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないって。マイク大丈夫。 | 0.00 | es | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | Dijeron que no se escuchaba la voz. El micrófono está bien. | 65 | 3672 | 972 |
| c36 | 笑わないでよ！ | 笑わないでよ。 | 0.00 | es | ¡No se rían! | ¡No reírte! | 17 | 2857 | 2157 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です | 0.00 | es | Soy de la tercera generación de hololive. | Soy de la tercera generación de Hololive. | 100 | 4254 | 2674 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて。 | 0.00 | es | La próxima no voy a fallar, así que miren. | La próxima no fallaré, así que mirad. | 63 | 4531 | 2921 |
| c39 | この敵、倒さなくてもいいよ。 | この敵、倒さなくてもいいよ。 | 0.00 | es | No hace falta derrotar a este enemigo. | ¡Este enemigo, no es necesario derrotarlo! | 49 | 4450 | 2690 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっけ? | 0.00 | es | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estábamos hablando? | 84 | 4751 | 2811 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | es | Ayer hice una colaboración con la capitana Marine, y... | Ayer hice un collab con el capitán Marine. | 40 | 5171 | 3091 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | めっちゃ楽しかったこのゲーム前にもやったことあるんだよね | 0.47 | es | Ya jugué este juego antes, ¿sabes? | ¡Me encantó este juego, ya lo había jugado antes! | 43 | 6079 | 3149 |
| p2b | でも全然覚えてない。 |  | 1.00 | - |  |  | 0 | - | - |
| p3a | 犯人はたぶんあの人だと思う。 | でも全然覚えてない。犯人は多分あの人だと思う。 | 0.92 | es | Creo que el culpable probablemente es esa persona. | Pero no recuerdo nada. Probablemente esa persona es la culpable. | 63 | 5018 | 3488 |
| p3b | いや、違うかも。 |  | 1.00 | - |  |  | 0 | - | - |
| p4a | 宿題やった？ | いや、違うかも。宿題あった。 | 1.40 | es | ¿Hiciste la tarea? | No, quizás no. Tenía tarea. | 27 | 4007 | 3227 |
| p4b | まだやってない。 | まだやってない。 | 0.00 | es | Todavía no la hice. | Aún no lo hice. | 27 | 3252 | 2612 |
