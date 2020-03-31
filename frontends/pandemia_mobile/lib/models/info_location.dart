// this code is autogenerated by ansvia-vscode extension.
// please don't edit this by hand
// use 'ansvia-vscode extension > Edit Model fields' instead.
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/record.dart';

/// Model for InfoLocation
@immutable
class InfoLocation extends Equatable {
  final int id;
  final String name;
  final Record latestRecord;
  final List<Record> history;

  InfoLocation(this.id, this.name, this.latestRecord, this.history)
      : super();

  Map<String, dynamic> toMap() {
    Map<String, dynamic> data = Map();
    data["id"] = this.id;
    data["name"] = this.name;
    data["latest_record"] =
        this.latestRecord != null ? this.latestRecord.toMap() : null;
    data["history"] = this.history != null
        ? this.history.map((a) => a.toMap()).toList()
        : List();
    return data;
  }

  static InfoLocation fromMap(Map<String, dynamic> data) {
    assert(data['name'] != null, "InfoLocation.name is null");
    assert(data['latest_record'] != null, "InfoLocation.latest_record is null");
    return InfoLocation(
        data['id'] as int,
        data['name'] as String,
        data['latest_record'] != null
            ? Record.fromMap(data['latest_record'])
            : null,
        data['history'] != null
            ? List.from(data['history'].map((a) => Record.fromMap(a)).toList())
            : []);
  }

  InfoLocation copy({String name, Record latestRecord, List<Record> history}) {
    return InfoLocation(this.id, name ?? this.name,
        latestRecord ?? this.latestRecord, history ?? this.history);
  }

  @override
  List<Object> get props => [id, name, latestRecord, history];
}
