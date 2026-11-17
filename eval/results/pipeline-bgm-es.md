# Pipeline run: `eval/data/tts/bgm.wav`

- corpus CER (ASR vs truth): 0.221
- mean chrF en: None, es: Some(52.8)
- speech start -> subtitle (en): n/a
- speech end -> subtitle (en): n/a
- speech start -> subtitle (es): n=42 median 2924 / p95 3900 / max 6082
- speech end -> subtitle (es): n=42 median 1310 / p95 1655 / max 3661
- ASR ms: n=45 median 365 / p95 524 / max 557
- MT ms en: n/a | es: n=45 median 287 / p95 520 / max 693
- MT queue wait en: n/a | es: n=45 median 0 / p95 0 / max 0
- segments 45 (forced cuts 0, blips 0), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 0 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 45, translations: 45, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: []
- stop: 56 ms (joined true)
- capture: CaptureStats { packets: 12572, frames: 6034268, dropped_packets: 0, discontinuities: 0, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよ | 0.18 | es | Pekora no va a jugar un juego de terror hoy. | Pekora no va a jugar un juego de terror hoy. | 100 | 3825 | 1655 |
| c02 | え、待って、それ本当？ | え、待って、それ本当… | 0.00 | es | ¿Eh? Espera, ¿eso es verdad? | Eh, espera, ¿eso es verdad...? | 78 | 3015 | 1075 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい、死ぬ死ぬ。 | 0.00 | es | ¡No, no, no, me muero, me muero! | ¡Ay, ay, ay, voy a morir, voy a morir! | 9 | 3129 | 1419 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | es | No da nada de miedo. | ¡En absoluto me da miedo! | 24 | 2325 | 1005 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど、大丈夫。 | 0.00 | es | No es que no tenga miedo, pero estoy bien. | No es que no tenga miedo, pero estoy bien. | 100 | 3648 | 1438 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで、ちょっと待って。 | 0.00 | es | No te vayas todavía, espera un momento. | No vayas todavía, espera un poco. | 66 | 2838 | 1258 |
| c07 | みんな、コメントありがとう！ | みんな、コメントありがとう。 | 0.00 | es | ¡Gracias por los comentarios a todos! | ¡Gracias a todos por los comentarios! | 86 | 3190 | 1310 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ。 | 0.12 | es | Hoy vamos a jugar un juego nuevo, peko. | Hoy Pekora jugará a un nuevo juego. | 40 | 3351 | 1391 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長のせいだよ | 0.00 | es | Eso es culpa de la capitana Marine. | ¡Es culpa del capitán Marine! | 52 | 2647 | 1177 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの | 0.00 | es | ¿Quién dijo eso? | ¿Quién dijo eso? | 100 | 2502 | 1012 |
| c11 | もう二度とやらない。 | もう二度とやらない | 0.00 | es | No lo vuelvo a hacer nunca más. | ¡Ya no lo haré otra vez! | 14 | 2040 | 1130 |
| c12 | あれ、ドアが開かない。 | あれ?ドアが開かない。 | 0.00 | es | ¿Eh? La puerta no se abre. | ¿Eh? La puerta no se abre. | 100 | 2493 | 1153 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう | 0.00 | es | Tomemos la llave y después volvamos. | ¡Primero me quitaré la llave y luego volveré! | 31 | 2789 | 1269 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん | 0.00 | es | ¡Te dije que era a la izquierda! | ¡Dije que era la izquierda! | 78 | 2103 | 1013 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | es | ¿No está el sonido un poco demasiado alto? | ¡Un poco, ¿no es demasiado fuerte el sonido? | 55 | 2522 | 1392 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | es | Perdón por empezar tarde la transmisión. | Lo siento, empecé el stream más tarde de lo previsto. | 22 | 3508 | 1378 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたから、ちょっと休憩する。 | 0.00 | es | Tengo hambre, así que voy a tomar un descansito. | ¡Tengo hambre, voy a hacer una pequeña pausa! | 37 | 3900 | 1580 |
| c18 | これ、絶対罠でしょ。 | これ、絶対罠でしょ | 0.00 | es | Esto seguro que es una trampa, ¿no? | ¡Esto, definitivamente es una trampa! | 42 | 2517 | 1117 |
| c19 | 勝った！やったー！ | 勝ったやったー | 0.00 | es | ¡Gané! ¡Bien! | ¡Gané! ¡Qué buena! | 49 | 1403 | -287 |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど | 0.00 | es | Es que no quiero perder. | No quería perder. | 40 | 2486 | 1396 |
| c21 | 知らなかった、そんなの。 | 知らなかったそんなの | 0.00 | es | No sabía eso. | ¡No sabía eso! | 86 | 2438 | 1028 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ | 0.33 | es | Eso no es de Pekora. | Eso no es Pekora. | 66 | 2778 | 1268 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから見に来てね | 0.00 | es | Mañana también voy a transmitir, así que vengan a ver. | ¡También ven mañana a la transmisión! | 39 | 3611 | 1341 |
| c24 | え、なんで？意味わかんない。 | え、なんで?意味わかんない | 0.00 | es | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No tengo idea de qué significa. | 42 | 1817 | -263 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直しだ先輩に怒られちゃった | 0.62 | es | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si muero aquí, tengo que empezar de nuevo y el senpai se enojará. | 69 | 6082 | 3661 |
| c26 | 先輩に怒られちゃった。 |  | 1.00 | - |  |  | 0 | - | - |
| c27 | 別に好きじゃないし。 |  | 1.00 | - |  |  | 0 | - | - |
| c28 | お疲れぺこ！また明日！ | 別に好きじゃないしお疲れペコまた明日 | 1.22 | es | ¡Buen trabajo, peko! ¡Hasta mañana! | No me gusta nada y gracias por el esfuerzo, Pekora, mañana. | 29 | 3880 | 1480 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | es | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63 | 2578 | 1358 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない | 0.10 | es | Todavía no he ganado ni una vez. | ¡Aún no he comprado una sola vez! | 23 | 2228 | 1168 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう。助かる。 | 0.00 | es | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat! Me alivia. | 63 | 3456 | 1396 |
| c32 | あ、そこ宝箱あるじゃん。 | あ、そこ宝箱あるじゃん | 0.00 | es | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí! | 42 | 2924 | 1224 |
| c33 | 行けると思ったのに。 |  | 1.00 | - |  |  | 0 | - | - |
| c34 | 嘘でしょ、また落ちた。 | いけると思ったのに嘘でしょまた落ちた | 1.00 | es | No puede ser, me caí otra vez. | ¡Pensé que podía ir, pero es mentira, ¡me caí otra vez! | 46 | 3030 | 1310 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないってマイク大丈夫 | 0.00 | es | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | Dijeron que no se podía oír el sonido. ¿El micrófono está bien? | 47 | 2233 | -467 |
| c36 | 笑わないでよ！ | 笑わないでよ | 0.00 | es | ¡No se rían! | ¡No reírte! | 17 | 1687 | 987 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です | 0.00 | es | Soy de la tercera generación de hololive. | Soy de la tercera generación de Hololive. | 100 | 2840 | 1260 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて | 0.00 | es | La próxima no voy a fallar, así que miren. | ¡La próxima no fallaré, así que mirad! | 62 | 3073 | 1463 |
| c39 | この敵、倒さなくてもいいよ。 | この敵、倒さなくてもいいよ | 0.00 | es | No hace falta derrotar a este enemigo. | ¡Este enemigo, no es necesario derrotarlo! | 49 | 3090 | 1330 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっけ? | 0.00 | es | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estaba hablando? | 54 | 3292 | 1352 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | es | Ayer hice una colaboración con la capitana Marine, y... | Ayer hice un collab con el capitán Marine. | 40 | 3456 | 1376 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | めっちゃ楽しかったこのゲーム前にもやったことあるんだよね | 0.47 | es | Ya jugué este juego antes, ¿sabes? | ¡Me encantó este juego, ya lo había jugado antes! | 43 | 4174 | 1244 |
| p2b | でも全然覚えてない。 |  | 1.00 | - |  |  | 0 | - | - |
| p3a | 犯人はたぶんあの人だと思う。 | でも全然覚えてない。犯人は多分あの人だと思う。 | 0.92 | es | Creo que el culpable probablemente es esa persona. | Pero no recuerdo nada. Probablemente esa persona es la culpable. | 63 | 3083 | 1553 |
| p3b | いや、違うかも。 | いや違うかも宿題あった | 0.83 | es | No, tal vez no. | No, quizás no. Tenía tarea. | 21 | 3890 | 2560 |
| p4a | 宿題やった？ |  | 1.00 | - |  |  | 0 | - | - |
| p4b | まだやってない。 | まだやってない | 0.00 | es | Todavía no la hice. | Aún no lo hice. | 27 | 1728 | 1088 |
