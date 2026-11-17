# MT evaluation

model: `C:\Users\jhvan\AppData\Local\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf`  
corpus: `eval/corpus/conversational.jsonl`  
context: true  
spanish route: EnglishAssisted

## ja-es

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora no va a jugar un juego de terror hoy. | Pekora no juega un juego de terror hoy. | 71.5 | 1493 |
| c02 | え、待って、それ本当？ | ¿Eh? Espera, ¿eso es verdad? | ¿Eh, espera, ¿eso es verdad? | 85.6 | 511 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | ¡No, no, no, me muero, me muero! | ¡Ay, ay, ay, voy a morir! | 6.7 | 708 |
| c04 | 全然怖くないし。 | No da nada de miedo. | ¡No tiene miedo en absoluto! | 28.0 | 457 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | No es que no tenga miedo, pero estoy bien. | No es tan aterrador, pero estoy bien. | 49.4 | 595 |
| c06 | まだ行かないで、ちょっと待って。 | No te vayas todavía, espera un momento. | ¡No vayas todavía, espera un poco! | 64.8 | 562 |
| c07 | みんな、コメントありがとう！ | ¡Gracias por los comentarios a todos! | ¡Gracias a todos por los comentarios! | 86.3 | 468 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Hoy vamos a jugar un juego nuevo, peko. | ¡Hoy Pekora va a jugar a un nuevo juego! | 39.6 | 712 |
| c09 | それはマリン船長のせいだよ。 | Eso es culpa de la capitana Marine. | ¡Eso es culpa del capitán Marine! | 62.3 | 519 |
| c10 | 誰がそんなこと言ったの？ | ¿Quién dijo eso? | ¿Quién dijo eso? | 100.0 | 332 |
| c11 | もう二度とやらない。 | No lo vuelvo a hacer nunca más. | ¡Ya no lo haré otra vez! | 13.7 | 495 |
| c12 | あれ、ドアが開かない。 | ¿Eh? La puerta no se abre. | ¡Eh, la puerta no se abre! | 75.2 | 545 |
| c13 | 鍵を取ってから戻ろう。 | Tomemos la llave y después volvamos. | ¡Primero tomemos la llave y volvamos! | 63.3 | 641 |
| c14 | 左だって言ったじゃん！ | ¡Te dije que era a la izquierda! | ¡Dije izquierda! | 42.8 | 340 |
| c15 | ちょっと音大きすぎない？ | ¿No está el sonido un poco demasiado alto? | ¿Un poco, ¿no es demasiado fuerte el volumen? | 40.8 | 557 |
| c16 | 配信始めるの遅れてごめんね。 | Perdón por empezar tarde la transmisión. | Disculpa, empecé el stream tarde. | 21.2 | 616 |
| c17 | お腹すいたから、ちょっと休憩する。 | Tengo hambre, así que voy a tomar un descansito. | ¡Tengo hambre, así que me daré un pequeño descanso! | 55.9 | 749 |
| c18 | これ、絶対罠でしょ。 | Esto seguro que es una trampa, ¿no? | ¡Esto, definitivamente es una trampa! | 42.0 | 518 |
| c19 | 勝った！やったー！ | ¡Gané! ¡Bien! | ¡Ganamos! ¡Qué alegría! | 20.5 | 453 |
| c20 | 負けたくないんだけど。 | Es que no quiero perder. | No quiero perder, pero... | 65.6 | 501 |
| c21 | 知らなかった、そんなの。 | No sabía eso. | No lo sabía, eso. | 55.1 | 428 |
| c22 | それはぺこらのじゃないよ。 | Eso no es de Pekora. | Eso no es de Pekora. | 100.0 | 469 |
| c23 | 明日も配信するから、見に来てね。 | Mañana también voy a transmitir, así que vengan a ver. | Mañana también haré una transmisión, así que ven a verla. | 61.3 | 710 |
| c24 | え、なんで？意味わかんない。 | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No entiendo el significado. | 47.3 | 721 |
| c25 | ここで死んだら最初からやり直しだ。 | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si muero aquí, tengo que empezar de nuevo. | 70.2 | 780 |
| c26 | 先輩に怒られちゃった。 | Mi senpai me regañó. | Me enojó el senpai. | 29.4 | 555 |
| c27 | 別に好きじゃないし。 | Tampoco es que me guste. | No les gusto mucho. | 17.0 | 399 |
| c28 | お疲れぺこ！また明日！ | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias, peko! ¡Mañana! | 37.1 | 591 |
| c29 | ボス強すぎるんだけど。 | Este jefe es demasiado fuerte. | ¡El jefe es demasiado fuerte! | 79.9 | 466 |
| c30 | まだ一回も勝ってない。 | Todavía no he ganado ni una vez. | ¡Aún no he ganado ni una sola vez! | 60.4 | 583 |
| c31 | スパチャありがとう、助かる！ | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat, me alivia! | 72.7 | 642 |
| c32 | あ、そこ宝箱あるじゃん。 | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí mismo! | 41.8 | 589 |
| c33 | 行けると思ったのに。 | Pensé que lo iba a lograr... | ¡Pensé que podía ir! | 31.0 | 443 |
| c34 | 嘘でしょ、また落ちた。 | No puede ser, me caí otra vez. | ¡Miente! Caíste otra vez. | 34.2 | 559 |
| c35 | 声が聞こえないって？マイク大丈夫？ | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | ¿No se oye tu voz? ¿El micrófono está bien? | 50.9 | 789 |
| c36 | 笑わないでよ！ | ¡No se rían! | ¡No rías! | 26.0 | 380 |
| c37 | ホロライブの3期生です。 | Soy de la tercera generación de hololive. | Soy una miembro de la tercera generación de Hololive. | 88.2 | 612 |
| c38 | 次は失敗しないから見てて。 | La próxima no voy a fallar, así que miren. | La próxima no fallaré, así que mirad. | 63.4 | 664 |
| c39 | この敵、倒さなくてもいいよ。 | No hace falta derrotar a este enemigo. | ¡Este enemigo no tienes que derrotarlo! | 49.4 | 593 |
| c40 | えっと、何の話だっけ？ | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estábamos hablando? | 84.1 | 581 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Ayer hice una colaboración con la capitana Marine, y... | Ayer hice un collab con el capitán Marine, pero... | 43.6 | 702 |
| p1b | めっちゃ楽しかった。 | Fue súper divertido. | ¡Fue súper divertido! | 92.3 | 428 |
| p2a | このゲーム、前にもやったことあるんだよね。 | Ya jugué este juego antes, ¿sabes? | ¡Este juego ya lo he jugado antes! | 43.2 | 556 |
| p2b | でも全然覚えてない。 | Pero no me acuerdo para nada. | Pero no recuerdo nada. | 46.0 | 459 |
| p3a | 犯人はたぶんあの人だと思う。 | Creo que el culpable probablemente es esa persona. | Creo que el culpable es probablemente esa persona. | 87.3 | 577 |
| p3b | いや、違うかも。 | No, tal vez no. | No, podría ser otra persona. | 14.3 | 473 |
| p4a | 宿題やった？ | ¿Hiciste la tarea? | ¿Hiciste la tarea? | 100.0 | 414 |
| p4b | まだやってない。 | Todavía no la hice. | Aún no la he hecho. | 22.7 | 469 |

mean chrF 53.8; latency ms median 557 / p95 780 / max 1493; load 3122 ms (prepare 30710 ms wall)

