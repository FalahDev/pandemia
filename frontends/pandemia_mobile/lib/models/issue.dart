// this code is autogenerated by ansvia-vscode extension.
// please don't edit this by hand
// use 'ansvia-vscode extension > Edit Model fields' instead.
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

/// Model for Issue
@immutable
class Issue extends Equatable {
  final String id;
  final String name;
  final String classification;
  final int classificationNum;
  final String creatorName;
  final String desc;
  final String primaryImage;
  final String registerTime;

  Issue(this.id, this.name, this.classification, this.classificationNum,
      this.creatorName, this.desc, this.primaryImage, this.registerTime)
      : super([
          id,
          name,
          classification,
          classificationNum,
          creatorName,
          desc,
          primaryImage,
          registerTime
        ]);

  Map<String, dynamic> toMap() {
    Map<String, dynamic> data = Map();
    data["id"] = this.id;
    data["name"] = this.name;
    data["classification"] = this.classification;
    data["classification_num"] = this.classificationNum;
    data["creator_name"] = this.creatorName;
    data["desc"] = this.desc;
    data["primary_image"] = this.primaryImage;
    data["register_time"] = this.registerTime;
    return data;
  }

  static Issue fromMap(Map<String, dynamic> data) {
    assert(data['name'] != null, "Issue.name is null");
    assert(data['classification'] != null, "Issue.classification is null");
    assert(
        data['classification_num'] != null, "Issue.classification_num is null");
    assert(data['creator_name'] != null, "Issue.creator_name is null");
    assert(data['desc'] != null, "Issue.desc is null");
    assert(data['primary_image'] != null, "Issue.primary_image is null");
    assert(data['register_time'] != null, "Issue.register_time is null");
    return Issue(
        data['id'] as String,
        data['name'] as String,
        data['classification'] as String,
        data['classification_num'] as int,
        data['creator_name'] as String,
        data['desc'] as String,
        data['primary_image'] as String,
        data['register_time'] as String);
  }

  Issue copy(
      {String name,
      String classification,
      int classificationNum,
      String creatorName,
      String desc,
      String primaryImage,
      String registerTime}) {
    return Issue(
        this.id,
        name ?? this.name,
        classification ?? this.classification,
        classificationNum ?? this.classificationNum,
        creatorName ?? this.creatorName,
        desc ?? this.desc,
        primaryImage ?? this.primaryImage,
        registerTime ?? this.registerTime);
  }
}
