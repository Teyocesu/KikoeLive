# MT evaluation

model: `C:\Users\jhvan\AppData\Local\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf`  
corpus: `eval/corpus/conversational.jsonl`  
context: true  
spanish route: Direct

## ja-es

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora no va a jugar un juego de terror hoy. | Pekora no va a jugar un juego de terror hoy. | 100.0 | 731 |
| c02 | え、待って、それ本当？ | ¿Eh? Espera, ¿eso es verdad? | ¿Eh, espera, ¿eso es verdad? | 85.6 | 301 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | ¡No, no, no, me muero, me muero! | ¡Ay, ay, ay, me muero, me muero! | 63.4 | 411 |
| c04 | 全然怖くないし。 | No da nada de miedo. | ¡No tiene miedo en absoluto! | 28.0 | 208 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | No es que no tenga miedo, pero estoy bien. | No es que no tenga miedo, pero estoy bien. | 100.0 | 326 |
| c06 | まだ行かないで、ちょっと待って。 | No te vayas todavía, espera un momento. | ¡No vayas todavía, espera un poco! | 64.8 | 278 |
| c07 | みんな、コメントありがとう！ | ¡Gracias por los comentarios a todos! | ¡Gracias a todos por los comentarios! | 86.3 | 261 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Hoy vamos a jugar un juego nuevo, peko. | ¡Hoy Pekora jugará a un nuevo juego! | 39.1 | 324 |
| c09 | それはマリン船長のせいだよ。 | Eso es culpa de la capitana Marine. | ¡Es culpa del capitán Marine! | 52.0 | 230 |
| c10 | 誰がそんなこと言ったの？ | ¿Quién dijo eso? | ¿Quién dijo eso? | 100.0 | 173 |
| c11 | もう二度とやらない。 | No lo vuelvo a hacer nunca más. | ¡Ya no lo haré otra vez! | 13.7 | 255 |
| c12 | あれ、ドアが開かない。 | ¿Eh? La puerta no se abre. | ¡Eh, la puerta no se abre! | 75.2 | 278 |
| c13 | 鍵を取ってから戻ろう。 | Tomemos la llave y después volvamos. | ¡Primero me quito la llave y luego volveré. | 32.2 | 413 |
| c14 | 左だって言ったじゃん！ | ¡Te dije que era a la izquierda! | ¡Dije que era la izquierda! | 77.9 | 232 |
| c15 | ちょっと音大きすぎない？ | ¿No está el sonido un poco demasiado alto? | ¡Un poco, ¿no es demasiado fuerte el sonido? | 55.2 | 324 |
| c16 | 配信始めるの遅れてごめんね。 | Perdón por empezar tarde la transmisión. | Disculpa, empecé el stream tarde. | 21.2 | 300 |
| c17 | お腹すいたから、ちょっと休憩する。 | Tengo hambre, así que voy a tomar un descansito. | Tengo hambre, así que me daré un pequeño descanso. | 57.0 | 484 |
| c18 | これ、絶対罠でしょ。 | Esto seguro que es una trampa, ¿no? | ¡Esto, definitivamente es una trampa! | 42.0 | 301 |
| c19 | 勝った！やったー！ | ¡Gané! ¡Bien! | ¡Ganamos! ¡Qué bien! | 40.2 | 258 |
| c20 | 負けたくないんだけど。 | Es que no quiero perder. | No quiero perder, pero... | 65.6 | 187 |
| c21 | 知らなかった、そんなの。 | No sabía eso. | ¡No lo sabía, eso! | 47.4 | 218 |
| c22 | それはぺこらのじゃないよ。 | Eso no es de Pekora. | Eso no es de Pekora. | 100.0 | 239 |
| c23 | 明日も配信するから、見に来てね。 | Mañana también voy a transmitir, así que vengan a ver. | Mañana también haré una transmisión, así que ven a verla. | 61.3 | 390 |
| c24 | え、なんで？意味わかんない。 | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No entiendo el significado. | 47.3 | 366 |
| c25 | ここで死んだら最初からやり直しだ。 | Si muero aquí, tengo que empezar de nuevo desde el principio. | ¡Si muero aquí, tengo que empezar de nuevo! | 69.2 | 322 |
| c26 | 先輩に怒られちゃった。 | Mi senpai me regañó. | ¡El senpai me enojó. | 41.0 | 258 |
| c27 | 別に好きじゃないし。 | Tampoco es que me guste. | No me gusta nada. | 24.2 | 167 |
| c28 | お疲れぺこ！また明日！ | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias, Pekora! ¡Mañana otra vez! | 30.7 | 342 |
| c29 | ボス強すぎるんだけど。 | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63.3 | 218 |
| c30 | まだ一回も勝ってない。 | Todavía no he ganado ni una vez. | ¡Aún no he ganado ni una vez. | 72.6 | 279 |
| c31 | スパチャありがとう、助かる！ | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat! ¡Me salvó! | 61.9 | 349 |
| c32 | あ、そこ宝箱あるじゃん。 | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí, ¿no? | 41.5 | 332 |
| c33 | 行けると思ったのに。 | Pensé que lo iba a lograr... | ¡Pensé que podía ir. | 32.6 | 232 |
| c34 | 嘘でしょ、また落ちた。 | No puede ser, me caí otra vez. | ¡Miente! ¡Caí otra vez! | 38.7 | 276 |
| c35 | 声が聞こえないって？マイク大丈夫？ | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | ¿No se oye tu voz? ¿El micrófono está bien? | 50.9 | 435 |
| c36 | 笑わないでよ！ | ¡No se rían! | ¡No rías! | 26.0 | 165 |
| c37 | ホロライブの3期生です。 | Soy de la tercera generación de hololive. | Soy de la tercera generación de Hololive. | 100.0 | 308 |
| c38 | 次は失敗しないから見てて。 | La próxima no voy a fallar, así que miren. | La próxima no fallaré, así que mirad. | 63.4 | 315 |
| c39 | この敵、倒さなくてもいいよ。 | No hace falta derrotar a este enemigo. | ¡Este enemigo, no es necesario derrotarlo! | 49.5 | 278 |
| c40 | えっと、何の話だっけ？ | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estábamos hablando? | 84.1 | 451 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Ayer hice una colaboración con la capitana Marine, y... | ¡Ay! ¡Ayer hice un collab con el capitán Marine, pero! | 41.5 | 455 |
| p1b | めっちゃ楽しかった。 | Fue súper divertido. | ¡Fue súper divertido! | 92.3 | 205 |
| p2a | このゲーム、前にもやったことあるんだよね。 | Ya jugué este juego antes, ¿sabes? | ¡Este juego ya lo he jugado antes, ¿verdad? | 48.5 | 376 |
| p2b | でも全然覚えてない。 | Pero no me acuerdo para nada. | Pero no recuerdo nada de nada. | 44.9 | 217 |
| p3a | 犯人はたぶんあの人だと思う。 | Creo que el culpable probablemente es esa persona. | Creo que el culpable es probablemente esa persona. | 87.3 | 297 |
| p3b | いや、違うかも。 | No, tal vez no. | No, quizás no sea esa persona. | 15.3 | 243 |
| p4a | 宿題やった？ | ¿Hiciste la tarea? | ¿Hiciste la tarea? | 100.0 | 308 |
| p4b | まだやってない。 | Todavía no la hice. | Aún no lo he hecho. | 13.0 | 211 |

mean chrF 57.2; latency ms median 297 / p95 455 / max 731; load 2367 ms (prepare 17021 ms wall)

