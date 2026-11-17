# Pipeline run: `eval/data/tts/bgm.wav`

- corpus CER (ASR vs truth): 0.221
- mean chrF en: Some(64.6), es: None
- speech start -> subtitle (en): n=42 median 2979 / p95 3992 / max 6099
- speech end -> subtitle (en): n=42 median 1279 / p95 1634 / max 3678
- speech start -> subtitle (es): n/a
- speech end -> subtitle (es): n/a
- ASR ms: n=45 median 363 / p95 512 / max 527
- MT ms en: n=45 median 261 / p95 505 / max 665 | es: n/a
- MT queue wait en: n=45 median 0 / p95 0 / max 0 | es: n/a
- segments 45 (forced cuts 0, blips 0), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 0 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 45, translations: 45, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: []
- stop: 9 ms (joined true)
- capture: CaptureStats { packets: 12572, frames: 6034268, dropped_packets: 0, discontinuities: 0, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよ | 0.18 | en | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100 | 3794 | 1624 |
| c02 | え、待って、それ本当？ | え、待って、それ本当… | 0.00 | en | Wait, what? Is that true? | Wait, really? | 18 | 3029 | 1089 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい、死ぬ死ぬ。 | 0.00 | en | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die. | 69 | 3085 | 1375 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | en | It's not scary at all. | Not scary at all! | 71 | 2282 | 962 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど、大丈夫。 | 0.00 | en | It's not that I'm not scared, but I'm okay. | It's not exactly scary, but I'm okay. | 50 | 3603 | 1393 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで、ちょっと待って。 | 0.00 | en | Don't go yet, wait a second. | Don't go yet, wait a little. | 67 | 2801 | 1221 |
| c07 | みんな、コメントありがとう！ | みんな、コメントありがとう。 | 0.00 | en | Thanks for the comments, everyone! | Thanks for all the comments, everyone! | 89 | 3159 | 1279 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ。 | 0.12 | en | Today we're going to play a new game, peko. | Today, Pekora is going to play a new game. | 67 | 3444 | 1484 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長のせいだよ | 0.00 | en | That's Captain Marine's fault. | That's Captain Marine's fault. | 100 | 2597 | 1127 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの | 0.00 | en | Who said that? | Who said that? | 100 | 2479 | 989 |
| c11 | もう二度とやらない。 | もう二度とやらない | 0.00 | en | I'm never doing this again. | I won't do it again. | 26 | 1963 | 1053 |
| c12 | あれ、ドアが開かない。 | あれ?ドアが開かない。 | 0.00 | en | Huh, the door won't open. | Wait, the door won't open. | 83 | 2419 | 1079 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう | 0.00 | en | Let's get the key and then go back. | I'll go back and get the key. | 46 | 2755 | 1235 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん | 0.00 | en | I told you it was left! | I told you to go left! | 51 | 2068 | 978 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | en | Isn't the sound a bit too loud? | The sound is a bit too loud. | 65 | 2368 | 1238 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | en | Sorry for starting the stream late. | Sorry, I started the stream a bit late. | 49 | 3473 | 1343 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたから、ちょっと休憩する。 | 0.00 | en | I'm hungry, so I'll take a short break. | My stomach's rumbling, so I'm taking a short break. | 47 | 3765 | 1445 |
| c18 | これ、絶対罠でしょ。 | これ、絶対罠でしょ | 0.00 | en | This is definitely a trap, right? | This is definitely a trap, right? | 100 | 2458 | 1058 |
| c19 | 勝った！やったー！ | 勝ったやったー | 0.00 | en | I won! Yay! | I won Got it! | 22 | 1355 | -335 |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど | 0.00 | en | I really don't want to lose. | I don't want to lose, though. | 62 | 2593 | 1503 |
| c21 | 知らなかった、そんなの。 | 知らなかったそんなの | 0.00 | en | I didn't know that. | Didn't know that at all. | 81 | 2454 | 1044 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ | 0.33 | en | That's not Pekora's. | That's not Pekora at all. | 78 | 2785 | 1275 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから見に来てね | 0.00 | en | I'm streaming tomorrow too, so come watch. | Come watch the stream tomorrow, peko! | 48 | 3595 | 1325 |
| c24 | え、なんで？意味わかんない。 | え、なんで?意味わかんない | 0.00 | en | Huh, why? That makes no sense. | Wait, why? I don't get it. | 16 | 1751 | -329 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直しだ先輩に怒られちゃった | 0.62 | en | If I die here, I have to start over from the beginning. | If I die here, I have to restart from the beginning, and the senpai will get mad. | 70 | 6099 | 3678 |
| c26 | 先輩に怒られちゃった。 |  | 1.00 | - |  |  | 0 | - | - |
| c27 | 別に好きじゃないし。 |  | 1.00 | - |  |  | 0 | - | - |
| c28 | お疲れぺこ！また明日！ | 別に好きじゃないしお疲れペコまた明日 | 1.22 | en | Good work today, peko! See you tomorrow! | I don't really like it, thanks for watching Pekora, see you tomorrow. | 41 | 3992 | 1592 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | en | This boss is way too strong. | The boss is way too strong, though. | 74 | 2581 | 1361 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない | 0.10 | en | I haven't won even once yet. | I haven't bought it even once yet. | 70 | 2187 | 1127 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう。助かる。 | 0.00 | en | Thanks for the superchat, that really helps! | Thanks for the superchat. I'm so relieved. | 56 | 3381 | 1321 |
| c32 | あ、そこ宝箱あるじゃん。 | あ、そこ宝箱あるじゃん | 0.00 | en | Oh, there's a treasure chest right there. | Oh, there's a treasure chest over there, right? | 79 | 3068 | 1368 |
| c33 | 行けると思ったのに。 |  | 1.00 | - |  |  | 0 | - | - |
| c34 | 嘘でしょ、また落ちた。 | いけると思ったのに嘘でしょまた落ちた | 1.00 | en | No way, I fell again. | I thought I could go, but that's a lie, I fell again. | 50 | 3033 | 1313 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないってマイク大丈夫 | 0.00 | en | You can't hear my voice? Is the mic okay? | I can't hear the voice. Is the mic okay? | 64 | 2119 | -581 |
| c36 | 笑わないでよ！ | 笑わないでよ | 0.00 | en | Don't laugh! | Don't laugh at me. | 80 | 1689 | 989 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です | 0.00 | en | I'm part of hololive's third generation. | I'm a third-generation hololive member. | 51 | 2800 | 1220 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて | 0.00 | en | I won't mess up next time, so watch. | I won't fail this time, so watch! | 50 | 2979 | 1369 |
| c39 | この敵、倒さなくてもいいよ。 | この敵、倒さなくてもいいよ | 0.00 | en | You don't have to defeat this enemy. | This enemy doesn't have to be defeated. | 60 | 3063 | 1303 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっけ? | 0.00 | en | Um, what were we talking about? | Um, what were we talking about? | 100 | 3309 | 1369 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | en | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday | 62 | 3463 | 1383 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | めっちゃ楽しかったこのゲーム前にもやったことあるんだよね | 0.47 | en | I've actually played this game before. | That game was super fun, and I've played it before. | 38 | 4031 | 1101 |
| p2b | でも全然覚えてない。 |  | 1.00 | - |  |  | 0 | - | - |
| p3a | 犯人はたぶんあの人だと思う。 | でも全然覚えてない。犯人は多分あの人だと思う。 | 0.92 | en | I think the culprit is probably that person. | But I don't remember anything at all. I think the culprit is probably that person. | 85 | 3164 | 1634 |
| p3b | いや、違うかも。 | いや違うかも宿題あった | 0.83 | en | No, maybe not. | No, maybe not. I had homework. | 78 | 3820 | 2490 |
| p4a | 宿題やった？ |  | 1.00 | - |  |  | 0 | - | - |
| p4b | まだやってない。 | まだやってない | 0.00 | en | I haven't done it yet. | I haven't done it yet. | 100 | 1719 | 1079 |
