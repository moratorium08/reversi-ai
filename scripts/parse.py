# coding:utf-8
import re
import sys


class GGFOthelloGame:
    def __init__(self, game_str):
        self.hands = []
        game_str = game_str[2:-2]
        reg = r"(.+?)\[(.+?)\]"
        for m in re.finditer(reg, game_str):
            signature = m.group(1)
            contents = m.group(2)
            if "GM" == signature:
                self.game_name = contents
                # sorry, only for othello
                if self.game_name != "Othello":
                    raise Exception("This parser is only supported for Othello")
            elif "PC" == signature:
                self._set_place(contents)
            elif "DT" == signature:
                self.date = contents
            elif "PB" == signature:
                self.black_player_name = contents
            elif "PW" == signature:
                self.white_player_name = contents
            elif "RB" == signature:
                self.black_player_rate = float(contents)
            elif "RW" == signature:
                self.white_player_rate = float(contents)
            elif "TI" == signature:
                self._set_thinking_time(contents)
            elif "TB" == signature:
                self._set_black_thinking_time(contents)
            elif "TY" == signature:
                self.game_type = contents
            elif "RE" == signature:
                if ":" in contents:
                    tmp = contents.split(":")
                    self.result = float(tmp[0])
                    self.result_flag = tmp[1]
                else:
                    self.result = float(contents)

                if self.result > 0:
                    self.winner = "B"  # black
                elif self.result == 0:
                    self.winner = "D"  # draw
                else:
                    self.winner = "W"  # white
            elif "BO" == signature:
                # only 8x8
                size = int(contents[0])
                if size != 8:
                    raise Exception("This parser supports only 8x8")
                contents = contents[1:]
                self._set_board(contents)
            elif "B" == signature:
                self._put(contents, 1)
            elif "W" == signature:
                self._put(contents, -1)
            elif "KB" == signature:
                # sorry mendoi
                raise Exception("Komi is not supported")
            elif "KW" == signature:
                raise Exception("Komi is not supported")
            elif "KM" == signature:
                raise Exception("Komi is not supported")
            else:
                raise Exception("Un supported Proc %s" % signature)

    def _set_place(self, contents):
        pass

    def _set_board(self, contents):
        rows = contents[:-1].strip(" ").split(" ")
        p, q = 2, 2
        for x, row in enumerate(rows):
            for y, val in enumerate(row):
                if val == "-":
                    pass
                elif val == "O":
                    p -= 1
                else:
                    q -= 1
        if p != 0 or q != 0:
            print(contents, x, y)
            raise Exception("not supported without regular start")

    def _put(self, contents, player):
        vals = contents.split("/")
        move = vals[0]
        if move == "pass":
            self.hands.append("pass")
            return
        self.hands.append(move[0] + move[1])

    def _set_thinking_time(self, contents):
        pass

    def _set_black_thinking_time(self, contents):
        raise NotImplementedError

    def _set_white_thinking_time(self, contents):
        raise NotImplementedError


def parse_ggf(filename):
    with open(filename, "r") as f:
        s = f.read().replace("\n", "")
        reg = r"\(;.+?;\)"
        games = []
        for m in re.finditer(reg, s):
            try:
                g = GGFOthelloGame(m.group(0))
                games.append(g)
            except:
                print(sys.exc_info())
    return games


if __name__ == '__main__':
    filename = "Othello.01e4.ggf"
    dump = "dump.csv"
    with open(dump, "w") as f:
        for game in parse_ggf(filename):
            s = game.winner + ',' + ','.join(game.hands) + '\n'
            f.write(s)
