# Pipeline run: `eval/data/tts/bgm.wav`

- corpus CER (ASR vs truth): 0.158
- mean chrF en: Some(61.8), es: None
- speech start -> subtitle (en): n=45 median 2122 / p95 3055 / max 4986
- speech end -> subtitle (en): n=45 median 744 / p95 1126 / max 2816
- speech start -> subtitle (es): n/a
- speech end -> subtitle (es): n/a
- ASR ms: n=54 median 276 / p95 388 / max 478
- MT ms en: n=54 median 251 / p95 398 / max 1132 | es: n/a
- MT queue wait en: n=54 median 0 / p95 0 / max 0 | es: n/a
- segments 54 (forced cuts 0, blips 12), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 0 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 54, translations: 54, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: []
- stop: 36 ms (joined true)
- capture: CaptureStats { packets: 12572, frames: 6034268, dropped_packets: 0, discontinuities: 0, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよえ? | 0.24 | en | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today, is she? | 93 | 4986 | 2816 |
| c02 | え、待って、それ本当？ | 待ってそれもんと | 0.50 | en | Wait, what? Is that true? | Wait No way | 11 | 1946 | 6 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい死ぬ死ぬ | 0.00 | en | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | 100 | 2983 | 1273 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | en | It's not scary at all. | It's not scary at all! | 93 | 2064 | 744 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど大丈夫。 | 0.00 | en | It's not that I'm not scared, but I'm okay. | It's not like it's not scary at all, but... It's fine. | 36 | 2279 | 69 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで | 0.50 | en | Don't go yet, wait a second. | Don't go yet. | 42 | 1408 | -172 |
| c07 | みんな、コメントありがとう！ | みんなコメントありがとう。 | 0.00 | en | Thanks for the comments, everyone! | Everyone Thanks for the comments. | 84 | 1142 | -738 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ | 0.12 | en | Today we're going to play a new game, peko. | Today, Peko will play a new game! | 47 | 2995 | 1035 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長の聖代。 | 0.31 | en | That's Captain Marine's fault. | That's Captain Marine's holy grail. | 75 | 2210 | 740 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの | 0.00 | en | Who said that? | Who said that? | 100 | 2203 | 713 |
| c11 | もう二度とやらない。 | もう二度とやらない | 0.00 | en | I'm never doing this again. | I won't do that again. | 24 | 1622 | 712 |
| c12 | あれ、ドアが開かない。 | ドアが開かない。 | 0.22 | en | Huh, the door won't open. | The door won't open. | 82 | 2050 | 710 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう | 0.00 | en | Let's get the key and then go back. | Let's go back and get the key. | 57 | 2379 | 859 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん。 | 0.00 | en | I told you it was left! | I told you to go left. | 46 | 1943 | 853 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | en | Isn't the sound a bit too loud? | The sound is a bit too loud. | 65 | 1929 | 799 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | en | Sorry for starting the stream late. | Sorry, I started the stream a bit late. | 49 | 3055 | 925 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたからちょっと休憩する | 0.00 | en | I'm hungry, so I'll take a short break. | I'm hungry. I'll take a short break. | 82 | 1646 | -674 |
| c18 | これ、絶対罠でしょ。 | 絶対罠でした。 | 0.38 | en | This is definitely a trap, right? | It was definitely a trap. | 57 | 2083 | 683 |
| c19 | 勝った！やったー！ |  | 1.00 | - |  |  | 0 | - | - |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど | 0.00 | en | I really don't want to lose. | I don't want to lose, but... | 62 | 2184 | 1094 |
| c21 | 知らなかった、そんなの。 | 知らなかった | 0.40 | en | I didn't know that. | I didn't know. | 67 | 1566 | 156 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ | 0.33 | en | That's not Pekora's. | That's not Pekora at all. | 78 | 2636 | 1126 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから見に来てね | 0.00 | en | I'm streaming tomorrow too, so come watch. | I'll be streaming tomorrow too. Come watch, peko! | 72 | 2273 | 3 |
| c24 | え、なんで？意味わかんない。 | 意味わかんない | 0.36 | en | Huh, why? That makes no sense. | I don't get it. | 4 | 2959 | 879 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直し | 0.06 | en | If I die here, I have to start over from the beginning. | If I die here, I have to start over from the beginning. | 100 | 3291 | 870 |
| c26 | 先輩に怒られちゃった。 | 先輩に怒られちゃった | 0.00 | en | I got scolded by my senpai. | I got scolded by senpai. | 82 | 2122 | 772 |
| c27 | 別に好きじゃないし。 | 別に好きじゃない | 0.11 | en | It's not like I like it or anything. | I don't really like it. | 21 | 1563 | 553 |
| c28 | お疲れぺこ！また明日！ | お疲れまた明日。 | 0.22 | en | Good work today, peko! See you tomorrow! | Thanks for watching! See you tomorrow. | 44 | 1449 | -951 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | en | This boss is way too strong. | The boss is way too strong though. | 75 | 2220 | 1000 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない。 | 0.10 | en | I haven't won even once yet. | I haven't bought it even once yet. | 70 | 1912 | 852 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう。 | 0.25 | en | Thanks for the superchat, that really helps! | Thanks for the superchat. | 57 | 1879 | -181 |
| c32 | あ、そこ宝箱あるじゃん。 | そこ宝箱あるじゃん。 | 0.10 | en | Oh, there's a treasure chest right there. | There's a treasure chest there, right? | 76 | 2717 | 1017 |
| c33 | 行けると思ったのに。 | いけると思ったのに | 0.11 | en | I thought I could make it... | I thought I could go in. | 60 | 1861 | 911 |
| c34 | 嘘でしょ、また落ちた。 | 嘘でしょ | 0.56 | en | No way, I fell again. | That's a lie. | 6 | 1510 | -210 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないってマイク大丈夫 | 0.00 | en | You can't hear my voice? Is the mic okay? | I thought I could hear you, but that's a lie. The mic is fine. | 26 | 2173 | -527 |
| c36 | 笑わないでよ！ | 笑わないでよ | 0.00 | en | Don't laugh! | Don't laugh at me. | 80 | 1477 | 777 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です。 | 0.00 | en | I'm part of hololive's third generation. | I'm a third-gen Hololive member. | 34 | 2453 | 873 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて。 | 0.00 | en | I won't mess up next time, so watch. | Next time I won't fail, so watch this. | 54 | 2576 | 966 |
| c39 | この敵、倒さなくてもいいよ。 | 倒さなくてもいい。 | 0.33 | en | You don't have to defeat this enemy. | You don't have to defeat it. | 69 | 2489 | 729 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっ | 0.11 | en | Um, what were we talking about? | Um, what were we talking about? | 100 | 2702 | 762 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | en | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday. | 62 | 2933 | 853 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | このゲーム前にもやったことあるんだよね | 0.00 | en | I've actually played this game before. | This game I've played this game before. | 67 | 1403 | -1527 |
| p2b | でも全然覚えてない。 | でも全然覚えてない。 | 0.00 | en | But I don't remember it at all. | But I don't remember anything about it. | 64 | 2095 | 865 |
| p3a | 犯人はたぶんあの人だと思う。 | 犯人は多分あの人だと思う | 0.23 | en | I think the culprit is probably that person. | I think the culprit is probably that person. | 100 | 2234 | 704 |
| p3b | いや、違うかも。 | いや違うかも。 | 0.00 | en | No, maybe not. | No way. Maybe not. | 67 | 1107 | -223 |
| p4a | 宿題やった？ | 宿題やった。 | 0.00 | en | Did you do your homework? | I did my homework. | 37 | 1418 | 638 |
| p4b | まだやってない。 |  | 1.00 | - |  |  | 0 | - | - |
